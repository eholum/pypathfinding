use crate::graph::Graph;
use crate::pose::Pose;
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;
use pyo3::prelude::PyResult;
use pyo3::pyfunction;

/// Calls pathfindings astar algorithm with the specified start and goal poses,
/// using the provided graph.
#[pyfunction]
pub fn py_astar(start: &Pose, goal: &Pose, g: &Graph) -> PyResult<Option<(Vec<Pose>, f64)>> {
    let successors_func = |node: &Pose| -> Vec<(Pose, OrderedFloat<f64>)> { g.successors(node) };

    let heuristic_func = |node: &Pose| -> OrderedFloat<f64> { node.distance(goal) };

    let success_func = |node: &Pose| -> bool { node == goal };

    let result = astar(start, successors_func, heuristic_func, success_func);

    Ok(result.map(|(path, cost)| (path, cost.into_inner())))
}
