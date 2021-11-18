use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let mut possible_moves = vec![
        (pos.0 + 1, pos.1 + 2),
        (pos.0 + 2, pos.1 + 1),
        (pos.0 + 2, pos.1 - 1),
        (pos.0 + 1, pos.1 - 2),
        (pos.0 - 1, pos.1 - 2),
        (pos.0 - 2, pos.1 - 1),
        (pos.0 - 2, pos.1 + 1),
        (pos.0 - 1, pos.1 + 2),
    ];
    for (x, y) in possible_moves.iter_mut() {
        if *x < 0 || *x > 7 || *y < 0 || *y > 7 {
            continue;
        }
        let piece = board.get_piece((*x, *y));
        if piece.is_none() {
            moves.push((*x, *y));
            continue;
        }
        if piece.unwrap().get_color() != color {
            moves.push((*x, *y));
        }
    }
    moves
}
