use std::default::Default;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, collections::HashMap, f32::consts::PI};

use eframe::egui::{
    pos2, vec2, CentralPanel, CollapsingHeader, Frame, Key, Label, RichText, SidePanel, TextStyle,
    Widget, Window,
};
use eframe::{
    egui::{accesskit::Vec2, containers, Context, TopBottomPanel},
    epaint::Color32,
    wgpu::Instance,
    CreationContext, Storage,
};
use lazy_static::lazy_static;
use uuid::{uuid, Uuid};

use tidal_core::demo::Demo;
use tidal_core::graph::{Graph, NodeId};
use tidal_core::interpreter::interpreter::Interpreter;

use crate::interpreter_holder::InterpreterHolder;
use crate::node_editor::node_editor::{NodeEditorWidget, NodeEditorWidgetResponse};
use crate::node_inspector::{InspectorWidgetResponse, NodeInspectorWidget};
use crate::project::project::{Project, ProjectState};
use crate::project::recent::RecentProjects;
use crate::project::save_dialog::ProjectSaveDialog;
use crate::state::store::{Dispatcher, Store};
use crate::state::State;
use crate::viewport::ViewportWidget;

pub enum Focus {
    GraphEditor,
    Screen,
}

pub struct App {
    focused: Focus,
    // project: Arc<Mutex<Project>>,
    store: Store,

    node_editor_widget: NodeEditorWidget,
    node_inspector_widget: Option<NodeInspectorWidget>,
    viewport_widget: ViewportWidget,

    interpreter_holder: InterpreterHolder,
    save_dialog: ProjectSaveDialog,
    recent_projects: RecentProjects,
}

impl App {
    pub fn new(cc: &CreationContext) -> App {
        let a = NodeId::from(1);
        // let b = NodeId::new();
        // let c = NodeId::new();

        let mut graph = Graph::default();

        let interpreter_holder = InterpreterHolder::new();
        let viewport_widget = ViewportWidget::new(cc, interpreter_holder.clone());

        let recent_projects = cc
            .storage
            .map(|s| RecentProjects::load(s))
            .unwrap_or_default();

        App {
            focused: Focus::GraphEditor,
            // project: Arc::new(Mutex::new(Project::new(ProjectState { graph }))),
            store: Store::new(State { graph }),
            node_editor_widget: Default::default(),
            node_inspector_widget: Default::default(),
            viewport_widget,
            interpreter_holder,
            save_dialog: Default::default(),
            recent_projects,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        ctx.request_repaint();
        ctx.set_pixels_per_point(2.0);

        // if ctx.input(|i| i.key_down(Key::S) && i.modifiers.command) {
        //     self.save_dialog.open()
        // }

        // self.save_dialog.show(ctx, &mut project);

        self.interpreter_holder.load_demo(Demo {
            graph: self.store.state().graph.clone(),
        });

        SidePanel::right("side panel")
            .min_width(500.0)
            .max_width(1000.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.viewport_widget.show(ui);

                CollapsingHeader::new("inspector").show(ui, |ui| {
                    if let Some(node_inspector) = &mut self.node_inspector_widget {
                        let responses = node_inspector.show(ui, &self.store);
                        for response in responses {
                            match response {
                                InspectorWidgetResponse::Close => {
                                    self.node_inspector_widget = None;
                                }
                            }
                        }
                    }
                });
            });

        CentralPanel::default()
            .frame(Frame {
                fill: Color32::BLACK,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let responses = self.node_editor_widget.show(ui, &self.store);
                for response in responses {
                    match response {
                        NodeEditorWidgetResponse::OpenInspector { node_id } => {
                            self.node_inspector_widget = Some(NodeInspectorWidget { node_id })
                        }
                    }
                }
            });

        self.store.run();

        if ctx.input(|i| i.key_pressed(Key::Z) && i.modifiers.ctrl) {
            self.store.undo()
        } else if ctx.input(|i| i.key_pressed(Key::Y) && i.modifiers.ctrl) {
            self.store.redo()
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        self.recent_projects.store(storage);
    }
}
