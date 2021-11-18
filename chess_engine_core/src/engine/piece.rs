use crate::{Board, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color) => color.clone(),
            Piece::Rook(color) => color.clone(),
            Piece::Knight(color) => color.clone(),
            Piece::Bishop(color) => color.clone(),
            Piece::Queen(color) => color.clone(),
            Piece::King(color) => color.clone(),
        }
    }

    pub fn get_pseudo_legal_moves(&self, board: Board, pos: (i8, i8)) -> Vec<(i8, i8)> {
        match self {
            Piece::Pawn(color) => return crate::logic::pawn::logic(board, pos, *color),
            Piece::Knight(color) => return crate::logic::knight::logic(board, pos, *color),
            Piece::Bishop(color) => return crate::logic::bishop::logic(board, pos, *color),
            Piece::Rook(color) => return crate::logic::rook::logic(board, pos, *color),
            Piece::Queen(color) => return crate::logic::queen::logic(board, pos, *color),
            Piece::King(color) => return crate::logic::king::logic(board, pos, *color),
        }
    }
}
