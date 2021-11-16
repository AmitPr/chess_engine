mod engine;
mod bindings_py;
use bindings_py::{pyboard::PyBoard, pypiece::PyPiece, pycolor::PyColor};
use pyo3::{prelude::*};

/// Module definition. PyO3 will construct our module from this function.
#[pymodule]
fn chess_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBoard>()?;
    m.add_class::<PyColor>()?;
    m.add_class::<PyPiece>()?;

    Ok(())
}
