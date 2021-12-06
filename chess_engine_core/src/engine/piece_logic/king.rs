use crate::{Board, Color};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();

    if color == Color::White {
        if board.castling.white[1] {
            if board.castling.white[0] {
                let can_castle = [(0, 1), (0, 2), (0, 3)]
                    .iter()
                    .all(|&pos| board.get_piece(pos).is_none());
                if can_castle {
                    moves.push((0, 2));
                }
            }
            if board.castling.white[2] {
                let can_castle = [(0, 5), (0, 6)]
                    .iter()
                    .all(|&pos| board.get_piece(pos).is_none());
                if can_castle {
                    moves.push((0, 6));
                }
            }
        }
    } else {
        if board.castling.black[1] {
            if board.castling.black[0] {
                let can_castle = [(7, 1), (7, 2), (7, 3)]
                    .iter()
                    .all(|&pos| board.get_piece(pos).is_none());
                if can_castle {
                    moves.push((7, 2));
                }
            }
            if board.castling.black[2] {
                let can_castle = [(7, 5), (7, 6)]
                    .iter()
                    .all(|&pos| board.get_piece(pos).is_none());
                if can_castle {
                    moves.push((7, 6));
                }
            }
        }
    }

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let x = pos.0 + i;
            let y = pos.1 + j;
            if x < 0 || x > 7 || y < 0 || y > 7 {
                continue;
            }
            if board.get_piece_at(x, y).is_none() {
                moves.push((x, y));
            } else if board.get_piece_at(x, y).unwrap().get_color() != color {
                moves.push((x, y));
            }
        }
    }
    //TODO: castling
    moves
}
