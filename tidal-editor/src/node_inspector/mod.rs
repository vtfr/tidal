use std::fmt::format;
use std::fs::metadata;
use std::ops::RangeInclusive;
use std::vec;

use derive_more::Constructor;
use eframe::egui::panel::Side;
use eframe::egui::{
    vec2, Align, Align2, Area, CollapsingHeader, Color32, DragValue, Grid, Layout, Margin,
    RichText, SidePanel, Slider, Ui, Vec2, Widget,
};

use tidal_core::cgmath::Vector3;
use tidal_core::graph::node::{Constant, InputState};
use tidal_core::graph::{Graph, Metadata, Node, NodeId, NodePortId, PortId};

use crate::state::graph::GraphCommand;
use crate::state::store::Store;

pub struct NodeInspectorWidget {
    pub node_id: NodeId,
}

#[derive(Debug)]
pub enum InspectorWidgetResponse {
    Close,
}

impl NodeInspectorWidget {
    pub fn show(&mut self, ui: &mut Ui, store: &Store) -> Vec<InspectorWidgetResponse> {
        let graph = &store.state().graph;

        let Some(node) = graph.get_node(self.node_id) else {
            return vec![InspectorWidgetResponse::Close];
        };

        let metadata = node.operator.describe();

        ui.set_width(ui.available_width());

        let mut responses = vec![];

        self.show_title(ui, &metadata, &mut responses);
        self.show_inputs(ui, self.node_id, node, &metadata, &mut responses, store);

        responses
    }

    fn show_title(
        &self,
        ui: &mut Ui,
        metadata: &Metadata,
        _responses: &mut Vec<InspectorWidgetResponse>,
    ) {
        ui.vertical(|ui| {
            ui.heading(&*metadata.name);
            ui.add_space(10.0);
        });
    }

    fn show_inputs(
        &mut self,
        ui: &mut Ui,
        node_id: NodeId,
        node: &Node,
        metadata: &Metadata,
        responses: &mut Vec<InspectorWidgetResponse>,
        store: &Store,
    ) {
        Self::wrap_ports(ui, "Inputs", |ui| {
            Grid::new("ports").num_columns(2).show(ui, |ui| {
                for (port_id, state, input_metadata) in node.iter_described_inputs() {
                    ui.label(&*input_metadata.name);

                    match state {
                        InputState::Constant(c) => {
                            let updated_constant = match c {
                                Constant::Scalar(value) => {
                                    let mut value = *value;
                                    let mut changed = false;

                                    ui.horizontal(|ui| {
                                        changed |=
                                            DragValue::new(&mut value).speed(0.1).ui(ui).changed()
                                    });

                                    changed.then_some(Constant::Scalar(value))
                                }
                                Constant::Vector(v) => {
                                    let mut x = v.x;
                                    let mut y = v.y;
                                    let mut z = v.z;

                                    let mut changed = false;

                                    ui.horizontal(|ui| {
                                        changed |=
                                            DragValue::new(&mut x).speed(0.1).ui(ui).changed();
                                        changed |=
                                            DragValue::new(&mut y).speed(0.1).ui(ui).changed();
                                        changed |=
                                            DragValue::new(&mut z).speed(0.1).ui(ui).changed();
                                    });

                                    changed.then_some(Constant::Vector((Vector3::new(x, y, z))))
                                }
                                Constant::I32(_) => None,
                            };

                            if let Some(constant) = updated_constant {
                                store.dispatch(GraphCommand::ChangeConstant {
                                    node_id,
                                    port_id,
                                    constant,
                                });
                            }
                        }
                        InputState::Connection(_) => {
                            ui.label("connected :)");
                        }
                    };

                    ui.end_row();
                }
            });
        });
    }

    // fn show_outputs(&self, metadata: &Metadata, ui: &mut Ui) {
    //     Self::wrap_ports(ui, "Outputs", |ui| {});
    // }

    fn wrap_ports(ui: &mut Ui, heading: &str, add_contents: impl FnOnce(&mut Ui)) {
        let heading = RichText::new(heading).heading();

        CollapsingHeader::new(heading)
            .default_open(true)
            .show(ui, add_contents);
    }
}
