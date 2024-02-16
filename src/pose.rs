use ordered_float::OrderedFloat;
use pyo3::{pyclass, pymethods};
use std::fmt;

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

impl fmt::Display for Pose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[pymethods]
impl Pose {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Pose {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
        }
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (*self.x, *self.y)
    }

    pub fn print(&self) -> String {
        self.to_string()
    }

    pub fn x(&self) -> f64 {
        self.x.into_inner()
    }

    pub fn y(&self) -> f64 {
        self.y.into_inner()
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
