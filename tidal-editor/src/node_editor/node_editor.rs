use std::collections::{HashMap, HashSet};

use derive_more::IsVariant;
use eframe::egui::{
    pos2, vec2, CentralPanel, Color32, CursorIcon, Frame, Id, InnerResponse, Key, LayerId, Margin,
    Order, Painter, PointerButton, Pos2, Rect, Response, Rounding, Sense, Shape, SidePanel, Stroke,
    Ui, Vec2,
};
use eframe::epaint::RectShape;

use tidal_core::cgmath::Vector2;
use tidal_core::graph::{Graph, NodeId, NodePortId, Placement};

use crate::containers::zoom::Zoom;
use crate::drag::MinimumDrag;
use crate::node_editor::connection::{ConnectionWidget, ConnectionWidgetResponse};
use crate::node_editor::node::{NodeResponse, NodeWidget};
use crate::node_editor::node_editor_state::{
    CandidateInputCollector, InputCollector, Interaction, LinkCollector, NodeCollector,
    NodeEditorState, OutputCollector, PanZoom, TransientState,
};
use crate::node_editor::picker::{PickerResponse, PickerWidget};
use crate::rgba;
use crate::state::graph::GraphCommand;
use crate::state::store::{Dispatcher, Store};

#[derive(Debug, Default, Copy, Clone)]
pub enum NodeEditorResponse {
    Inspect(NodeId),
    #[default]
    None,
}

#[derive(Default, Debug)]
pub struct NodeEditorWidget {
    state: NodeEditorState,
    picker_widget: Option<PickerWidget>,
}

#[derive(Debug)]
pub enum NodeEditorWidgetResponse {
    OpenInspector { node_id: NodeId },
}

impl NodeEditorWidget {
    pub fn show(&mut self, ui: &mut Ui, store: &Store) -> Vec<NodeEditorWidgetResponse> {
        let graph = &store.state().graph;

        let mut node_editor_responses = vec![];

        let mut transient_state = TransientState::default();
        let mut is_hovering_node = false;
        let mut delayed_node_responses = vec![];

        let available_size = ui.available_size_before_wrap();

        // Pan to the center of the canvas.
        let pan_zoom = self.state.pan_zoom.with_adjusted_pan({
            let available_rect = ui.available_rect_before_wrap();
            let center_pan = available_rect.center() - available_rect.left_top();
            center_pan
        });

        let InnerResponse {
            response: canvas_response,
            ..
        } = Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_min_size(available_size);
            ui.set_max_size(available_size);
            ui.set_clip_rect(ui.max_rect());

            // Draw background grid
            Self::draw_background(ui, pan_zoom);

            // Allocate connections shape
            let connections_shape_idx = ui.painter().add(Shape::Noop);

            // Paint the selection rect if selecting
            if let Interaction::Selecting { start, end } = self.state.interaction {
                let selection_rect = Rect::from_points(&[start, end]);

                ui.painter().add(RectShape {
                    rect: selection_rect,
                    rounding: Rounding::ZERO,
                    fill: rgba!("61789E44"),
                    stroke: Stroke::new(2.0, rgba!("61789EFF")),
                    fill_texture_id: Default::default(),
                    uv: Rect::ZERO,
                });
            }

            Zoom::new(pan_zoom.zoom)
                .clip_rect(ui.clip_rect())
                .show(ui, |ui| {
                    Frame::canvas(ui.style()).show(ui, |ui| {
                        for (node_id, node) in graph.iter_nodes() {
                            let node_widget = NodeWidget {
                                graph,
                                pan_zoom,
                                editor_state: &mut self.state,
                                transient_state: &mut transient_state,
                            };

                            let node_responses = node_widget.show(ui, node_id, node);
                            for node_response in node_responses {
                                delayed_node_responses.push((node_id, node_response));
                            }
                        }
                    });

                    // Determine which is the closest input-placement pair to connect
                    self.calculate_closest_input_placement(graph, &mut transient_state, ui);

                    // Draw connections
                    let mut connection_widget = ConnectionWidget {
                        interaction: &self.state.interaction,
                        connections_shape_idx,
                        transient: &transient_state,
                    };

                    match connection_widget.show(ui) {
                        ConnectionWidgetResponse::Disconnected { input, index } => {
                            store.dispatch(GraphCommand::Disconnect { input, index })
                        }
                        ConnectionWidgetResponse::Default => {}
                    };
                });

            self.draw_picker_widget(ui, store);
        });

        // Process interactions
        for (node_id, node_response) in delayed_node_responses {
            match node_response {
                NodeResponse::OpenInspector => {
                    node_editor_responses.push(NodeEditorWidgetResponse::OpenInspector { node_id })
                }
                NodeResponse::StartConnecting(port_id) => {
                    self.state.interaction = Interaction::Connecting {
                        output: NodePortId(node_id, port_id),
                        candidate: None,
                    }
                }
                NodeResponse::FinishedConnecting => {
                    if let Interaction::Connecting { output, candidate } =
                        &mut self.state.interaction
                    {
                        if let Some((input, placement)) = *candidate {
                            store.dispatch(GraphCommand::ConnectNode {
                                output: *output,
                                input,
                                placement,
                            })
                        }
                    }

                    self.state.interaction = Interaction::Default
                }
                NodeResponse::Drag(delta) => {
                    // If we don't have any node selected and this node isn't part of the selected
                    // ones, then we simply unselect them all and move this node only.
                    if self.state.selected_nodes.is_empty()
                        || !self.state.selected_nodes.contains(&node_id)
                    {
                        self.state.selected_nodes.clear();

                        store.dispatch(GraphCommand::MoveNodes {
                            node_ids: HashSet::from([node_id]),
                            delta,
                        });
                    }
                    // If multiple, check if this node is part of the selected nodes.
                    else {
                        store.dispatch(GraphCommand::MoveNodes {
                            node_ids: self.state.selected_nodes.clone(),
                            delta,
                        });
                    }
                }
                NodeResponse::Hovering => is_hovering_node = true,
            }
        }

        self.process_interaction(
            ui,
            &mut transient_state,
            is_hovering_node,
            canvas_response,
            pan_zoom,
        );

        node_editor_responses
    }

    fn draw_picker_widget(&mut self, ui: &mut Ui, store: &Store) {
        if let Some(picker_widget) = &self.picker_widget {
            match picker_widget.show(ui) {
                PickerResponse::Close => {
                    self.picker_widget = None;
                }
                PickerResponse::CreateNode { operator, position } => {
                    store.dispatch(GraphCommand::CreateNode { operator, position });

                    self.picker_widget = None;
                }
                PickerResponse::None => {}
            }
        }
    }

    fn calculate_closest_input_placement(
        &mut self,
        graph: &Graph,
        mut transient_state: &mut TransientState,
        ui: &mut Ui,
    ) {
        if let Interaction::Connecting { output, candidate } = &mut self.state.interaction {
            if let Some(pointer) = ui.input(|i| i.pointer.hover_pos()) {
                *candidate = transient_state
                    .link_candidates
                    .iter()
                    .filter_map(|(input_placement, pos)| {
                        let distance = pos.distance(pointer);

                        if distance <= 10.0 && graph.can_connect(output, &input_placement.0) {
                            Some((input_placement, distance as i32))
                        } else {
                            None
                        }
                    })
                    .min_by_key(|(candidate, distance)| *distance)
                    .map(|(candidate, _)| *candidate);
            }
        }
    }

    fn draw_background(ui: &mut Ui, pan_zoom: PanZoom) {
        let rect = ui.max_rect();

        let lines_distance = 100.0;
        let line_increment = lines_distance * pan_zoom.zoom;
        let lines_x = (rect.width() / line_increment).ceil() as i32;
        let lines_y = (rect.height() / line_increment).ceil() as i32;
        let start_x = rect.left() as i32 + (pan_zoom.pan.x as i32) % (line_increment as i32);
        let start_y = rect.top() as i32 + (pan_zoom.pan.y as i32) % (line_increment as i32);

        for i in 0..lines_x {
            let x = (start_x + i * line_increment as i32) as f32;

            ui.painter().add(Shape::LineSegment {
                points: [pos2(x, rect.top()), pos2(x, rect.bottom())],
                stroke: Stroke::new(4.0 * pan_zoom.zoom, rgba!("222222ff")),
            });
        }

        for i in 0..lines_y {
            let y = (start_y + i * line_increment as i32) as f32;

            ui.painter().add(Shape::LineSegment {
                points: [pos2(rect.left(), y), pos2(rect.right(), y)],
                stroke: Stroke::new(4.0 * pan_zoom.zoom, rgba!("222222ff")),
            });
        }
    }

    fn process_interaction(
        &mut self,
        ui: &mut Ui,
        transient_state: &mut TransientState,
        is_hovering_node: bool,
        canvas_response: Response,
        pan_zoom: PanZoom,
    ) {
        let scroll_delta = ui.input(|i| i.scroll_delta);
        let zoom_delta = ui.input(|i| i.zoom_delta());

        self.state.pan_zoom.adjust_zoom(zoom_delta - 1.0);
        self.state.pan_zoom.pan += scroll_delta;

        if ui.input(|i| i.modifiers.ctrl && i.key_pressed(Key::C)) {
            let pointer = ui.input(|i| i.pointer.hover_pos()).unwrap_or_default();
            let position = pan_zoom.descale_pos(pointer);

            self.picker_widget = Some(PickerWidget {
                position: pointer,
                create_at_position: Vector2::new(position.x, position.y),
            });
        }

        if !is_hovering_node {
            // Reprocess the response
            let canvas_response = canvas_response.interact(Sense::click_and_drag());

            // If holding control
            if ui.input(|i| i.modifiers.ctrl) {
                // Dragging: Pan
                if MinimumDrag::default()
                    .minimum_distance(10.0)
                    .handle(&canvas_response)
                    .dragged
                {
                    ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

                    self.state.pan_zoom.pan += canvas_response.drag_delta();
                }
            }
            // Selecting
            else if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if canvas_response.drag_started_by(PointerButton::Primary) {
                    self.state.selected_nodes.clear();

                    self.state.interaction = Interaction::Selecting {
                        start: pointer_pos,
                        end: pointer_pos,
                    }
                }

                if canvas_response.dragged_by(PointerButton::Primary) {
                    if let Interaction::Selecting { start, ref mut end } = self.state.interaction {
                        // Update the last end position
                        *end = pointer_pos;

                        // Calculate selected nodes
                        let selection_rect = Rect::from_points(&[start, *end]);
                        for (node_id, node_rect) in transient_state.node_collector.iter() {
                            if selection_rect.intersects(*node_rect) {
                                self.state.selected_nodes.insert(*node_id);
                            }
                        }
                    }
                }

                if canvas_response.drag_released_by(PointerButton::Primary) {
                    self.state.interaction = Interaction::Default;
                }
            }
            // Clicked elsewhere
            else if canvas_response.clicked() {
                // Remove selected nodes
                self.state.selected_nodes.clear();
            }
            // Drag with middle button
            else {
                // let pan = MinimumDrag::default()
                //     .minimum_distance(10.0)
                //     .pointer_button(PointerButton::Middle)
                //     .handle(&canvas_response)
                //     .delta;
                //
                // self.state.pan_zoom.pan += pan;
            }
        }
    }
}
