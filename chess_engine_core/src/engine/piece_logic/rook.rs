use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let mut x = pos.0;
    let mut y = pos.1;
    loop {
        x += 1;
        if x > 7 {
            break;
        }
        if board.get_piece((x, y)).is_some() {
            if board.get_piece((x, y)).unwrap().get_color() != color {
                moves.push((x, y));
            }
            break;
        }
        moves.push((x, y));
    }
    x = pos.0;
    loop {
        x -= 1;
        if x < 0 {
            break;
        }
        if board.get_piece((x, y)).is_some() {
            if board.get_piece((x, y)).unwrap().get_color() != color {
                moves.push((x, y));
            }
            break;
        }
        moves.push((x, y));
    }
    x = pos.0;
    loop {
        y += 1;
        if y > 7 {
            break;
        }
        if board.get_piece((x, y)).is_some() {
            if board.get_piece((x, y)).unwrap().get_color() != color {
                moves.push((x, y));
            }
            break;
        }
        moves.push((x, y));
    }
    y = pos.1;
    loop {
        y -= 1;
        if y < 0 {
            break;
        }
        if board.get_piece((x, y)).is_some() {
            if board.get_piece((x, y)).unwrap().get_color() != color {
                moves.push((x, y));
            }
            break;
        }
        moves.push((x, y));
    }
    moves
}