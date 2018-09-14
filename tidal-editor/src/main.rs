extern crate eframe;
extern crate static_assertions;

use eframe::{App, CreationContext, EventLoopBuilder, UserEvent};

// use crate::viewport::ViewportWidget;

mod app;
mod color;
mod compiler;
mod containers;
mod drag;
mod interpreter_holder;
mod node_editor;
mod node_inspector;
mod project;
mod resource;
mod spash;
mod state;
mod viewport;
mod widget;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some((1500.0, 1200.0).into()),
        multisampling: 0,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "Tidal Editor",
        options,
        Box::new(|ctx: &CreationContext| Box::new(app::App::new(ctx)) as Box<dyn App + 'static>),
    )
}
