use super::pypiece::PyPiece;
use chess_engine_core::Move;
use pyo3::prelude::*;
#[pyclass(name = "Move", module = "chess_engine")]
pub struct PyMove {
    pub inner: Move,
}
#[pymethods]
impl PyMove {
    #[new]
    fn new(
        from: (i8, i8),
        to: (i8, i8),
        promotion: Option<PyPiece>,
        captured: Option<(PyPiece, (i8, i8))>,
    ) -> PyResult<Self> {
        Ok(PyMove {
            inner: Move::new(
                from,
                to,
                promotion.map(|p| p.get()),
                captured.map(|(p, pos)| (p.get(), pos)),
            ),
        })
    }
}

impl PyMove{
    pub fn from_move(move_: Move) -> PyMove {
        PyMove { inner: move_ }
    }
}
