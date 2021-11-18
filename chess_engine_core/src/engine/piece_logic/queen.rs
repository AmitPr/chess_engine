use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let mut diag = crate::logic::bishop::logic(board, pos, color);
    let mut straight = crate::logic::rook::logic(board, pos, color);
    moves.append(&mut diag);
    moves.append(&mut straight);
    moves
}