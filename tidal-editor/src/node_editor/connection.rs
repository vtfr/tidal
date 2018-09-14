use std::iter;

use derive_more::Constructor;
use eframe::egui::layers::ShapeIdx;
use eframe::egui::{pos2, vec2, Color32, Painter, PointerButton, Pos2, Shape, Stroke, Ui};
use eframe::epaint::CubicBezierShape;

use tidal_core::graph::{NodeId, NodePortId, Placement, PortId};

use crate::node_editor::node_editor_state::{
    CandidateInputCollector, InputCollector, Interaction, LinkCollector, OutputCollector,
    TransientState,
};
use crate::state::graph::GraphCommand;
use crate::state::store::Dispatcher;

#[derive(Debug, Default)]
pub enum ConnectionWidgetResponse {
    Disconnected {
        input: NodePortId,
        index: usize,
    },
    #[default]
    Default,
}

#[derive(Constructor)]
pub(crate) struct ConnectionWidget<'a> {
    pub interaction: &'a Interaction,
    pub transient: &'a TransientState,
    pub connections_shape_idx: ShapeIdx,
}

impl<'a> ConnectionWidget<'a> {
    pub fn show(mut self, ui: &mut Ui) -> ConnectionWidgetResponse {
        let mut widget_response = ConnectionWidgetResponse::Default;

        let mut links: Vec<_> = self
            .transient
            .link_collector
            .iter()
            .flat_map(|connection| {
                let (output, input, index) = connection;
                let output_pos = *self.transient.output_collector.get(output)?;
                let input_pos = *self.transient.input_collector.get(&(*input, *index))?;

                Some((*connection, Self::calculate_spline(output_pos, input_pos)))
            })
            .collect();

        self.process_curve_interaction(ui, &mut links, &mut widget_response);

        let mut shapes: Vec<_> = links.into_iter().map(|(_, shape)| shape.into()).collect();

        if let Some(connecting_shape) = self.calculate_connecting_shape(ui) {
            shapes.push(connecting_shape)
        }

        ui.painter()
            .set(self.connections_shape_idx, Shape::Vec(shapes));

        widget_response
    }

    fn process_curve_interaction(
        &mut self,
        ui: &mut Ui,
        links: &mut Vec<((NodePortId, NodePortId, usize), CubicBezierShape)>,
        widget_response: &mut ConnectionWidgetResponse,
    ) {
        // Calculate only interactions when no current interaction is happening
        // if self.interaction.is_default() {
        //     return;
        // }

        // Calculate only interactions if we have a valid pointer
        let Some(pointer) = ui.input(|i| i.pointer.hover_pos()) else {
            return;
        };

        // Pre-calculate whether the middle button was clicked
        let middle_clicked = ui.input(|i| i.pointer.button_clicked(PointerButton::Middle));

        let closest_curve = links
            .iter_mut()
            .filter_map(|(connection, curve)| {
                if ui.rect_contains_pointer(curve.visual_bounding_rect()) {
                    let minimum_distance = curve
                        .flatten(None)
                        .into_iter()
                        .map(|point| point.distance(pointer) as i32)
                        .min()?;

                    (minimum_distance < 100).then_some((connection, curve, minimum_distance))
                } else {
                    None
                }
            })
            .min_by_key(|(_, _, distance)| *distance)
            .map(|(connection, curve, _)| (connection, curve));

        if let Some(((_, input, index), curve)) = closest_curve {
            curve.stroke.color = Color32::WHITE;

            if middle_clicked {
                *widget_response = ConnectionWidgetResponse::Disconnected {
                    input: *input,
                    index: *index,
                };
            }
        }
    }

    fn calculate_connecting_shape(&self, ui: &mut Ui) -> Option<Shape> {
        let Interaction::Connecting { output, candidate } = &self.interaction else {
            return None;
        };

        let output_pos = *self.transient.output_collector.get(output)?;
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());

        let end_pos = candidate
            .as_ref()
            .and_then(|candidate| self.transient.link_candidates.get(candidate).copied())
            .or(pointer_pos)?;

        let shape = Self::calculate_spline(output_pos, end_pos);
        Some(shape.into())
    }

    fn calculate_spline(mut start: Pos2, mut end: Pos2) -> CubicBezierShape {
        let middle_x = (end.x - start.x).abs() / 2.0;

        let c1 = start + vec2(middle_x, 0.0);
        let c2 = end - vec2(middle_x, 0.0);

        let points = [start, c1, c2, end];

        CubicBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke::new(4.0, Color32::DARK_GRAY),
        )
    }
}
