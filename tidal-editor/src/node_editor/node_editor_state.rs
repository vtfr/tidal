use std::collections::{HashMap, HashSet};

use derive_more::IsVariant;
use eframe::egui::{Pos2, Rect, Vec2};

use tidal_core::graph::{NodeId, NodePortId, Placement};

use crate::node_editor::node::NodeResponse;

pub(crate) type CandidateInputCollector = HashMap<(NodePortId, Placement), Pos2>;
pub(crate) type OutputCollector = HashMap<NodePortId, Pos2>;
pub(crate) type InputCollector = HashMap<(NodePortId, usize), Pos2>;
pub(crate) type LinkCollector = HashSet<(NodePortId, NodePortId, usize)>;
pub(crate) type NodeCollector = HashMap<NodeId, Rect>;

#[derive(Debug, Default, Clone, IsVariant)]
pub(crate) enum Interaction {
    Connecting {
        output: NodePortId,
        candidate: Option<(NodePortId, Placement)>,
    },
    Selecting {
        start: Pos2,
        end: Pos2,
    },
    #[default]
    Default,
}

#[derive(Default, Debug)]
pub(crate) struct NodeEditorState {
    pub interaction: Interaction,
    pub pan_zoom: PanZoom,
    pub picker_opened: bool,
    pub selected_nodes: HashSet<NodeId>,
}

#[derive(Debug, Default)]
pub(crate) struct TransientState {
    pub node_collector: NodeCollector,
    pub input_collector: InputCollector,
    pub output_collector: OutputCollector,
    pub link_collector: LinkCollector,
    pub link_candidates: CandidateInputCollector,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct PanZoom {
    pub pan: Vec2,
    pub zoom: f32,
}

impl PanZoom {
    pub fn adjusted_pan(&mut self, pan: Vec2) {
        self.pan += pan
    }

    pub fn adjust_zoom(&mut self, increment: f32) {
        let zoom = self.zoom + increment;
        let zoom = zoom.clamp(0.25, 5.0);
        self.zoom = zoom;
    }

    pub fn with_adjusted_pan(mut self, pan: Vec2) -> PanZoom {
        self.adjusted_pan(pan);
        self
    }
}

impl Default for PanZoom {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl PanZoom {
    pub fn transform_position(&self, position: Pos2) -> Pos2 {
        (self.pan + position.to_vec2() * self.zoom).to_pos2()
    }

    pub fn descale_pos(&self, pos: Pos2) -> Pos2 {
        ((pos - self.pan).to_vec2() / self.zoom).to_pos2()
    }

    pub fn descale_vector(&self, vector: Vec2) -> Vec2 {
        vector / self.zoom
    }

    pub fn scale_vector(&self, vector: Vec2) -> Vec2 {
        vector * self.zoom
    }
}
