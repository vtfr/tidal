use derive_more::Constructor;
use eframe::egui::{
    vec2, Align, Align2, Area, Button, InnerResponse, Layout, Pos2, Rect, Shape, Ui, Vec2,
};
use eframe::epaint::RectShape;

use tidal_core::cgmath::{Vector2, Zero};
use tidal_core::operator::Operator;

use crate::rgba;
use crate::state::graph::GraphCommand;
use crate::state::store::Dispatcher;

#[derive(Debug)]
pub struct PickerWidget {
    pub create_at_position: Vector2<f32>,
    pub position: Pos2,
}

#[derive(Debug, Clone, Default)]
pub enum PickerResponse {
    Close,
    CreateNode {
        operator: Operator,
        position: Vector2<f32>,
    },
    #[default]
    None,
}

impl PickerWidget {
    pub fn show(&self, ui: &mut Ui) -> PickerResponse {
        let background_idx = ui.painter().add(Shape::Noop);

        let InnerResponse {
            inner: picker_response,
            ..
        } = Area::new("picker")
            .fixed_pos(self.position)
            .show(ui.ctx(), |ui| {
                for operator in Operator::all() {
                    if operator.0 != "Scene" {
                        if ui.button(&*operator.describe().name).clicked() {
                            return PickerResponse::CreateNode {
                                operator,
                                position: self.create_at_position,
                            };
                        }
                    }
                }

                PickerResponse::None
            });

        ui.painter().set(
            background_idx,
            RectShape {
                rect: ui.available_rect_before_wrap(),
                rounding: Default::default(),
                fill: rgba!("00000088"),
                stroke: Default::default(),
                fill_texture_id: Default::default(),
                uv: Rect::ZERO,
            },
        );

        picker_response
    }
}
