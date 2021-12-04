use super::pypiece::PyPiece;
use chess_engine_core::Move;
use pyo3::prelude::*;
#[pyclass(name = "Move", module = "chess_engine")]
#[derive(Copy, Clone, Debug)]
pub struct PyMove {
    pub inner: Move,
    pub from: (i8, i8),
    pub to: (i8, i8),
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
            from,
            to,
        })
    }

    #[getter]
    fn get_from_pos(&self) -> (i8, i8) {
        self.inner.from
    }
    #[setter]
    fn set_from_pos(&mut self, from: (i8, i8)) {
        self.inner.from = from;
        self.from = from;
    }

    #[getter]
    fn get_to_pos(&self) -> (i8, i8) {
        self.inner.to
    }
    #[setter]
    fn set_to_pos(&mut self, to: (i8, i8)) {
        self.inner.to = to;
        self.to = to;
    }
}

impl PyMove {
    pub fn from_move(move_: Move) -> PyMove {
        PyMove {
            inner: move_,
            from: move_.from,
            to: move_.to,
        }
    }

    pub fn get(&self) -> Move {
        self.inner.clone()
    }
}
