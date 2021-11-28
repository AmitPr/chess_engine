use pyo3::prelude::*;

use chess_engine_core::{Color, Piece};

use super::pycolor::PyColor;

#[pyclass(name = "Piece", module = "chess_engine")]
#[derive(Copy, Clone, Debug)]
pub struct PyPiece {
    pub inner: Piece,
}

#[pymethods]
impl PyPiece {
    #[getter]
    pub fn color(&self) -> PyColor {
        PyColor::new(self.inner.get_color())
    }

    pub fn piece_type(&self) -> String {
        match self.inner {
            Piece::Pawn(Color::White) => "Pawn".to_string(),
            Piece::Pawn(Color::Black) => "Pawn".to_string(),
            Piece::Rook(Color::White) => "Rook".to_string(),
            Piece::Rook(Color::Black) => "Rook".to_string(),
            Piece::Knight(Color::White) => "Knight".to_string(),
            Piece::Knight(Color::Black) => "Knight".to_string(),
            Piece::Bishop(Color::White) => "Bishop".to_string(),
            Piece::Bishop(Color::Black) => "Bishop".to_string(),
            Piece::Queen(Color::White) => "Queen".to_string(),
            Piece::Queen(Color::Black) => "Queen".to_string(),
            Piece::King(Color::White) => "King".to_string(),
            Piece::King(Color::Black) => "King".to_string(),
        }
    }
    pub fn __str__(&self) -> String {
        match self.inner {
            Piece::Pawn(Color::White) => "P".to_string(),
            Piece::Pawn(Color::Black) => "p".to_string(),
            Piece::Rook(Color::White) => "R".to_string(),
            Piece::Rook(Color::Black) => "r".to_string(),
            Piece::Knight(Color::White) => "N".to_string(),
            Piece::Knight(Color::Black) => "n".to_string(),
            Piece::Bishop(Color::White) => "B".to_string(),
            Piece::Bishop(Color::Black) => "b".to_string(),
            Piece::Queen(Color::White) => "Q".to_string(),
            Piece::Queen(Color::Black) => "q".to_string(),
            Piece::King(Color::White) => "K".to_string(),
            Piece::King(Color::Black) => "k".to_string(),
        }
    }
    pub fn __repr__(&self) -> String {
        match self.inner {
            Piece::Pawn(Color::White) => "Pawn(White)".to_string(),
            Piece::Pawn(Color::Black) => "Pawn(Black)".to_string(),
            Piece::Rook(Color::White) => "Rook(White)".to_string(),
            Piece::Rook(Color::Black) => "Rook(Black)".to_string(),
            Piece::Knight(Color::White) => "Knight(White)".to_string(),
            Piece::Knight(Color::Black) => "Knight(Black)".to_string(),
            Piece::Bishop(Color::White) => "Bishop(White)".to_string(),
            Piece::Bishop(Color::Black) => "Bishop(Black)".to_string(),
            Piece::Queen(Color::White) => "Queen(White)".to_string(),
            Piece::Queen(Color::Black) => "Queen(Black)".to_string(),
            Piece::King(Color::White) => "King(White)".to_string(),
            Piece::King(Color::Black) => "King(Black)".to_string(),
        }
    }

    // pub fn get_legal_moves(&self, board: &PyBoard, from: (i8, i8)) -> Vec<PyMove> {
    //     let mut moves = Vec::new();
    //     for move_ in self.inner.get_legal_moves(board.inner.clone(), from) {
    //         moves.push(PyMove::from_move(move_));
    //     }
    //     moves
    // }
}

impl PyPiece {
    pub fn new(piece: &Piece) -> PyPiece {
        PyPiece {
            inner: piece.clone(),
        }
    }
    pub fn get(&self) -> Piece {
        self.inner.clone()
    }
}
