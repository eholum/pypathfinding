use pyo3::prelude::*;

mod graph;
mod pose;
mod py_astar;

/// A Python module implemented in Rust.
#[pymodule]
fn pypathfinding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<graph::Graph>()?;
    m.add_class::<pose::Pose>()?;
    m.add_function(wrap_pyfunction!(py_astar::py_astar, m)?)?;
    Ok(())
}
