use std::sync::{Arc, Mutex};

use eframe::egui::{PaintCallback, PaintCallbackInfo, Rect};
use eframe::egui_wgpu::{Callback, CallbackResources, CallbackTrait};
use eframe::wgpu::RenderPass;

use tidal_core::interpreter::{Interpreter, InterpreterContext};

use crate::interpreter_holder::InterpreterHolder;
use crate::viewport::resource::ViewportCallbackResource;

pub(crate) struct ViewportCallback {
    interpreter_holder: InterpreterHolder,
}

impl ViewportCallback {
    #[inline]
    pub fn new(interpreter_holder: InterpreterHolder) -> Self {
        Self { interpreter_holder }
    }
}

impl CallbackTrait for ViewportCallback {
    fn paint<'a>(
        &'a self,
        _: PaintCallbackInfo,
        render_pass: &mut RenderPass<'a>,
        callback_resources: &'a CallbackResources,
    ) {
        let mut viewport_callback_resource = callback_resources
            .get::<ViewportCallbackResource>()
            .unwrap();

        let mut renderer = viewport_callback_resource.renderer.lock().unwrap();

        let _ = self.interpreter_holder.lock().run(&mut InterpreterContext {
            renderer: &mut renderer,
            render_target: &viewport_callback_resource.texture_view,
            frame: 0.0,
        });

        // Draw
        render_pass.set_pipeline(&viewport_callback_resource.pipeline);
        render_pass.set_vertex_buffer(
            0,
            viewport_callback_resource
                .full_screen_quad_vertex_buffer
                .slice(..),
        );
        render_pass.set_bind_group(0, &viewport_callback_resource.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
