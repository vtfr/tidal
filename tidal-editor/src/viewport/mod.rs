use std::sync::{Arc, Mutex};

use delegate::delegate;
use eframe::egui::{vec2, CollapsingHeader, Frame, InnerResponse, Response, Ui, Vec2, Widget};
use eframe::egui_wgpu::Callback;
use eframe::{egui, CreationContext};
use egui::Sense;

use tidal_core::demo::Demo;
use tidal_core::interpreter::Interpreter;

use crate::interpreter_holder::InterpreterHolder;
use crate::viewport::callback::ViewportCallback;
use crate::viewport::resource::{EguiRendererContext, ViewportCallbackResource};

mod callback;
mod resource;

pub struct ViewportWidget {
    interpreter_holder: InterpreterHolder,
}

impl ViewportWidget {
    pub fn new(ctx: &CreationContext, interpreter_holder: InterpreterHolder) -> Self {
        let render_state = ctx.wgpu_render_state.as_ref().expect("WGPU enabled");

        // Build the resource
        let resource = ViewportCallbackResource::new(render_state);

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(resource);

        Self { interpreter_holder }
    }

    pub(crate) fn show(&self, ui: &mut Ui) {
        CollapsingHeader::new("viewport")
            .default_open(true)
            .show(ui, |ui| self.show_inner(ui));
    }

    pub(crate) fn show_inner(&self, ui: &mut Ui) {
        const ASPECT: f32 = 1920.0 / 1080.0;

        let rect = ui.available_rect_before_wrap();

        // Choose based on the minimum
        let size = if rect.width() < rect.height() {
            Vec2::new(rect.width(), rect.width() / ASPECT)
        } else {
            Vec2::new(rect.height() * ASPECT, rect.height())
        };

        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

        Frame::canvas(ui.style()).show(ui, |ui| {
            let callback = ViewportCallback::new(self.interpreter_holder.clone());

            ui.painter()
                .add(Callback::new_paint_callback(rect, callback));
        });
    }
}
