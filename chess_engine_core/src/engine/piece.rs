use crate::Color;

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
}
