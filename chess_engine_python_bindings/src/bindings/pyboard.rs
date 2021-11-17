use pyo3::prelude::*;

use super::pypiece::PyPiece;
use chess_engine_core::Board;

#[pyclass(name = "Board", module = "chess_engine")]
pub struct PyBoard {
    inner: Board,
}
#[pymethods]
impl PyBoard {
    #[new]
    fn new() -> PyResult<Self> {
        let board = Board::new();
        Ok(PyBoard { inner: board })
    }

    fn get_piece_at(&self, row: usize, col: usize) -> PyResult<Option<PyPiece>> {
        let piece = self.inner.pieces[row][col];
        match piece {
            Some(piece) => Ok(Some(PyPiece::new(&piece))),
            None => Ok(None),
        }
    }

    fn get_board(&self) -> PyResult<Vec<Vec<Option<PyPiece>>>> {
        let pieces = self.inner.pieces;
        let mut result = Vec::new();
        for row in pieces.iter() {
            let mut row_vec = Vec::new();
            for piece in row.iter() {
                row_vec.push(piece.map(|p| PyPiece::new(&p)));
            }
            result.push(row_vec);
        }
        Ok(result)
    }
}
