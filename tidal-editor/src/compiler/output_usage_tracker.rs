use std::collections::{HashMap, HashSet};

use crate::graph::node::Input;
use crate::graph::{Graph, NodeId, NodePortId};

/// Keeps track of how many times an output has been used
pub(crate) struct OutputUsageTracker(HashMap<NodePortId, usize>);

impl OutputUsageTracker {
    pub fn new(graph: &Graph) -> Self {
        Self(Self::build_map(graph))
    }

    pub fn get_usage_for(&self, node_output_id: &NodePortId) -> usize {
        self.0.get(node_output_id).copied().unwrap_or(0)
    }

    fn build_map(graph: &Graph) -> HashMap<NodePortId, usize> {
        let mut map = HashMap::new();
        let mut seen = HashSet::new();
        let mut queue = Vec::new();

        if let Some(root) = graph.nodes.get(&NodeId::root()) {
            queue.push(root)
        }

        while let Some(node) = queue.pop() {
            seen.insert(node.id);

            for (_, state) in node.inputs_iter() {
                if let Input::Connection(children) = &*state {
                    for node_output_id in children.iter() {
                        if let Some(counter) = map.get_mut(node_output_id) {
                            *counter += 1;
                        } else {
                            map.insert(*node_output_id, 1);
                        }

                        if let Some(child) = graph.nodes.get(&node_output_id.get_node_id()) {
                            if !seen.contains(&child.id) {
                                queue.push(child);
                            }
                        }
                    }
                }
            }
        }

        map
    }
}
