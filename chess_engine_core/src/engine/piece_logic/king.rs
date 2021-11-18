use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let mut x = pos.0;
    let mut y = pos.1;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            x += i;
            y += j;
            if x < 0 || x > 7 || y < 0 || y > 7 {
                continue;
            }
            if board.get_piece_at(x, y).is_none() {
                moves.push((x, y));
            } else if board.get_piece_at(x, y).unwrap().get_color() != color {
                moves.push((x, y));
                break;
            } else {
                break;
            }
        }
    }
    moves
}
