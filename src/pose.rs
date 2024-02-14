use ordered_float::OrderedFloat;
use pyo3::{pyclass, pymethods};

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
