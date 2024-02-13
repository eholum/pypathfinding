use pathfinding::prelude::astar;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::wrap_pyfunction;

/// Calls pathfindings astar algorithm.
#[pyfunction]
fn py_astar(
    start: (i32, i32), // Hardcoded to tuples for now...
    successors: PyObject,
    heuristic: PyObject,
    success: PyObject,
    py: Python,
) -> PyResult<Option<(Vec<(i32, i32)>, i32)>> { // Hardcoding to i32...

    // Convert the closures to function pointers or adapt them as necessary
    let successors_func = |node: &(i32, i32)| -> Vec<((i32, i32), i32)> {
        let args = PyTuple::new(py, &[node]);
        let tmp = successors.call1(py, args);
        match tmp.and_then(|val| val.extract(py)) {
            Ok(ret) => ret,
            // Just ignore and return nothing...
            Err(e) => {
                e.print(py);
                Vec::new()
            }
        }
    };

    let heuristic_func = |node: &(i32, i32)| -> i32 {
        let args = PyTuple::new(py, &[node], );
        match heuristic.call1(py, args).and_then(|val| val.extract(py)) {
            Ok(ret) => ret,
            // Just ignore and return nothing...
            Err(e) => {
                e.print(py);
                0
            }
        }
    };

    let success_func = |node: &(i32, i32)| -> bool {
        let args = PyTuple::new(py, &[node]);
        match success.call1(py, args).and_then(|val| val.extract(py)) {
            Ok(ret) => ret,
            // Just ignore and return nothing...
            Err(e) => {
                e.print(py);
                false
            }
        }
    };

    // Call astar
    let result = astar(
        &start,
        successors_func,
        heuristic_func,
        success_func,
    );

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pypathfinding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_astar, m)?)?;
    Ok(())
}
