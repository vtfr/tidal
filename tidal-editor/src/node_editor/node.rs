use std::borrow::Cow;
use std::collections::HashSet;

use derive_more::Constructor;
use eframe::egui::{
    pos2, vec2, Align, Area, Color32, Id, InnerResponse, LayerId, Layout, Painter, Rect, Response,
    RichText, Sense, Shape, Stroke, TextStyle, Ui, Vec2,
};
use eframe::epaint::{CircleShape, RectShape};

use tidal_core::graph::PortId;
use tidal_core::graph::{
    DataType, Graph, InputMetadata, Metadata, Node, NodeId, NodePortId, Placement,
};
use tidal_core::operator::input::InputState;

use crate::drag::MinimumDrag;
use crate::node_editor::node_editor_state::{
    CandidateInputCollector, InputCollector, Interaction, LinkCollector, NodeCollector,
    NodeEditorState, OutputCollector, PanZoom, TransientState,
};
use crate::state::graph::GraphCommand;
use crate::{color, rgb, rgba};

#[derive(Debug, Copy, Clone)]
pub(crate) enum NodeResponse {
    OpenInspector,
    StartConnecting(PortId),
    FinishedConnecting,
    Drag(Vec2),
    Hovering,
}

pub(crate) struct NodeWidget<'a> {
    pub graph: &'a Graph,
    pub editor_state: &'a mut NodeEditorState,
    pub transient_state: &'a mut TransientState,
    pub pan_zoom: PanZoom,
}

impl<'a> NodeWidget<'a> {
    pub fn show(mut self, ui: &mut Ui, node_id: NodeId, node: &Node) -> Vec<NodeResponse> {
        let mut node_responses = vec![];

        let padding = self.pan_zoom.scale_vector(vec2(0.0, 5.0));
        let min_size = self.pan_zoom.scale_vector(vec2(200.0, 10.0));

        // Set initial position and add padding
        let position = pos2(node.position.x, node.position.y);
        let position =
            ui.max_rect().left_top() + self.pan_zoom.transform_position(position).to_vec2();

        // Allocate background
        let background_idx = ui.painter().add(Shape::Noop);

        // Start drawing
        let mut ui = ui.child_ui_with_id_source(
            Rect::from_min_size(position, min_size),
            Layout::default(),
            node_id,
        );

        let InnerResponse {
            response: contents_response,
            ..
        } = ui.vertical(|ui| {
            node_responses.extend(self.draw_title(ui, node));

            ui.add_space(ui.style().spacing.item_spacing.y);

            let layout = Layout::left_to_right(Align::Center).with_main_justify(true);

            ui.with_layout(layout, |ui| {
                self.draw_inputs(ui, node_id, node);

                node_responses.extend(self.draw_outputs(ui, node_id, node));
            });
        });

        // Draw background
        let contents_rect = contents_response.rect.expand2(padding);

        {
            // Draw stroke if selected
            let stroke = if self.editor_state.selected_nodes.contains(&node_id) {
                Stroke::new(2.0, rgba!("61789E88"))
            } else {
                Default::default()
            };

            ui.painter().set(
                background_idx,
                RectShape {
                    rect: contents_rect,
                    rounding: Default::default(),
                    fill: Color32::from_rgb(30, 30, 30),
                    stroke,
                    fill_texture_id: Default::default(),
                    uv: Rect::ZERO,
                },
            );
        }

        self.transient_state
            .node_collector
            .insert(node_id, contents_rect);

        // Interact for hovering
        if contents_response.hovered() {
            node_responses.push(NodeResponse::Hovering)
        }

        node_responses
    }

    fn draw_title(&mut self, ui: &mut Ui, node: &Node) -> Vec<NodeResponse> {
        let mut node_responses = vec![];

        let metadata = node.operator.describe();

        ui.centered_and_justified(|ui| {
            let label = RichText::new(&*metadata.name)
                .text_style(TextStyle::Heading)
                .strong();

            let response = ui.label(label).interact(Sense::drag());
            if response.dragged() {
                let delta = self.pan_zoom.descale_vector(response.drag_delta());

                node_responses.push(NodeResponse::Drag(delta));
            }

            if response.double_clicked() {
                node_responses.push(NodeResponse::OpenInspector);
            }
        });

        node_responses
    }

    fn draw_inputs(&mut self, ui: &mut Ui, node_id: NodeId, node: &Node) {
        ui.vertical(|ui| {
            // Only show all inputs when connecting nodes to something else
            // than the current node we're drawing.
            let show_all = if let Interaction::Connecting { output: from, .. } =
                &self.editor_state.interaction
            {
                from.get_node_id() != node_id
            } else {
                false
            };

            for (input_id, input_state, input_meta) in node.iter_described_inputs() {
                let input = NodePortId(node_id, input_id);

                // Collect links
                if let InputState::Connection(cs) = input_state {
                    for (index, output) in cs.iter().enumerate() {
                        self.transient_state
                            .link_collector
                            .insert((*output, input, index));
                    }
                }

                let input_placements = if show_all {
                    match input_state {
                        InputState::Constant(_) => vec![Placement::Replace(0)],
                        InputState::Connection(cs) => cs
                            .iter()
                            .copied()
                            .enumerate()
                            .flat_map(|(i, output)| {
                                vec![Placement::Insert(i), Placement::Replace(i)]
                            })
                            .chain(std::iter::once(Placement::Insert(cs.len())))
                            .collect(),
                    }
                } else {
                    match input_state {
                        InputState::Constant(_) => vec![],
                        InputState::Connection(cs) => cs
                            .iter()
                            .copied()
                            .enumerate()
                            .map(|(i, output)| Placement::Replace(i))
                            .collect(),
                    }
                };

                // Draw and collect their state
                let layout = Layout::left_to_right(Align::Min).with_cross_justify(true);

                if !input_placements.is_empty() {
                    ui.with_layout(layout, |ui| {
                        ui.vertical(|ui| {
                            for placement in input_placements {
                                let response = self.draw_port_symbol(ui, input_meta.data_type);
                                let rect = response.rect;

                                /// If this input maps to an output, then add it to the inputs collector.
                                /// This check is done so [`Placement::Insert`] input positions are not
                                /// counted.
                                if let Placement::Replace(index) = placement {
                                    self.transient_state
                                        .input_collector
                                        .insert((input, index), rect.center());
                                }

                                /// Collect candidates only if we're connecting
                                if matches!(
                                    self.editor_state.interaction,
                                    Interaction::Connecting { .. }
                                ) {
                                    self.transient_state
                                        .link_candidates
                                        .insert((input, placement), rect.center());
                                }
                            }
                        });

                        ui.label(&*input_meta.name);
                    });
                }
            }
        });
    }

    fn draw_outputs(&mut self, ui: &mut Ui, node_id: NodeId, node: &Node) -> Vec<NodeResponse> {
        let mut node_responses = vec![];
        let metadata = node.operator.describe();

        let layout = Layout::top_down(Align::Max);
        ui.with_layout(layout, |ui| {
            for (input_id, output_meta) in metadata.iter_outputs() {
                let node_port_id = NodePortId(node_id, input_id);

                ui.horizontal(|ui| {
                    let response = self
                        .draw_port_symbol(ui, output_meta.data_type)
                        .interact(Sense::click_and_drag());

                    ui.label(output_meta.name);

                    if response.drag_started() {
                        node_responses.push(NodeResponse::StartConnecting(input_id));
                    }
                    if response.drag_released() {
                        node_responses.push(NodeResponse::FinishedConnecting);
                    }

                    self.transient_state
                        .output_collector
                        .insert(node_port_id, response.rect.center());
                });
            }
        });

        node_responses
    }

    fn draw_port_symbol(&self, ui: &mut Ui, data_type: DataType) -> Response {
        const RADIUS: f32 = 5.0;
        const PADDING: f32 = 8.0;

        let fill = match data_type {
            DataType::Scalar => rgb!("709E68"),
            DataType::Vector => rgb!("6E9E9C"),
            DataType::Mesh => rgb!("57649E"),
            DataType::Texture => rgb!("9E4C52"),
            DataType::Command => rgb!("9E2A9E"),
        };

        let size = self.pan_zoom.scale_vector(Vec2::splat(RADIUS));
        let padded_size = self.pan_zoom.scale_vector(Vec2::splat(RADIUS + PADDING));

        let (rect, response) = ui.allocate_exact_size(padded_size, Sense::click_and_drag());

        ui.painter().add(CircleShape {
            center: rect.center(),
            radius: size.x,
            fill,
            stroke: Default::default(),
        });

        response
    }
}
