use crate::pose::Pose;
use ordered_float::OrderedFloat;
use pyo3::{pyclass, pymethods};
use std::collections::HashMap;

/// Basic graph class where nodes are points on the 2-D plane.
#[pyclass]
pub struct Graph {
    nodes: HashMap<Pose, Vec<(Pose, OrderedFloat<f64>)>>,
}

impl Graph {
    pub fn successors(&self, node: &Pose) -> Vec<(Pose, OrderedFloat<f64>)> {
        if let Some(successors) = self.nodes.get(node) {
            successors.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[pymethods]
impl Graph {
    #[new]
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, pose: Pose) {
        self.nodes.entry(pose).or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, from: Pose, to: Pose) {
        let d = from.distance(&to);
        self.nodes.entry(from).or_default().push((to, d));
    }
}
