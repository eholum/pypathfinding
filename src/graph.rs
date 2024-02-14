use ordered_float::OrderedFloat;
use pyo3::{pyclass, pymethods};
use std::collections::HashMap;

/// Simple pose class for storing x, y coordinates in the 2-D plane
/// 
/// Wraps the f64 primitive with OrderedFloat to support required interfaces from pathfinding.
#[pyclass]
#[derive(Clone, Hash)]
pub struct Pose {
    x: OrderedFloat<f64>,
    y: OrderedFloat<f64>,
}

impl PartialEq for Pose {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }
}

impl Eq for Pose {}

#[pymethods]
impl Pose {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Pose { x: OrderedFloat(x), y: OrderedFloat(y) }
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (*self.x, *self.y)
    }
}

impl Pose {
    /// Computes the euclidean distance between poses
    pub fn distance(&self, other: &Self) -> OrderedFloat<f64> {
        let xd = self.x - other.x;
        let yd = self.y - other.y;
        OrderedFloat((xd.powi(2) + yd.powi(2)).sqrt())
    }
}

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
