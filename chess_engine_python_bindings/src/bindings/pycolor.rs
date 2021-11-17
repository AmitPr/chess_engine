use pyo3::{basic::CompareOp, prelude::*};

use chess_engine_core::Color;

#[pyclass(name = "Color", module = "chess_engine")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyColor {
    inner: Color,
}

impl PyColor {
    pub fn new(inner: Color) -> PyColor {
        PyColor { inner }
    }
}

#[pymethods]
impl PyColor {
    #[classattr]
    #[allow(non_snake_case)]
    pub fn WHITE() -> Self {
        PyColor {
            inner: Color::White,
        }
    }
    #[classattr]
    #[allow(non_snake_case)]
    pub fn BLACK() -> Self {
        PyColor {
            inner: Color::Black,
        }
    }

    pub fn __repr__(&self) -> PyResult<String> {
        if self.inner == Color::White {
            Ok("Color.WHITE".to_string())
        } else {
            Ok("Color.BLACK".to_string())
        }
    }

    pub fn __richcmp__(&self, other: PyColor, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.inner == other.inner),
            CompareOp::Ne => Ok(self.inner != other.inner),
            _ => Ok(false),
        }
    }
}
