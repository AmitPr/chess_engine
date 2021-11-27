use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let direction = if color == Color::White { 1 } else { -1 };
    let forward = (pos.0 + direction, pos.1);
    if forward.0 < 0 || forward.0 > 7 {
        return moves;
    }
    if board.get_piece(forward).is_none() {
        moves.push(forward);
        let mut forward_two: Option<(i8, i8)> = None;
        if pos.0 == 1 && direction == 1 {
            forward_two = Some((pos.0 + 2 * direction, pos.1));
        } else if pos.0 == 6 && direction == -1 {
            forward_two = Some((pos.0 + 2 * direction, pos.1));
        }
        if forward_two.is_some() && board.get_piece(forward_two.unwrap()).is_none() {
            moves.push(forward_two.unwrap());
        }
    }
    let forward_left = (pos.0 + direction, pos.1 - 1);
    if forward_left.1 >= 0 {
        let piece = board.get_piece(forward_left);
        if let Some(piece) = piece {
            if piece.get_color() != color {
                moves.push(forward_left);
            }
        }
    }
    let forward_right = (pos.0 + direction, pos.1 + 1);
    if forward_right.1 < 8 {
        let piece = board.get_piece(forward_right);
        if let Some(piece) = piece {
            if piece.get_color() != color {
                moves.push(forward_right);
            }
        }
    }
    //TODO: En passant
    moves
}
