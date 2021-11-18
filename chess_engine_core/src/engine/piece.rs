use crate::{Board, Color, Move};

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

    pub fn get_legal_moves(&self, board: Board, pos: (i8, i8)) -> Vec<Move> {
        let mut moves = self
            .get_pseudo_legal_moves(board, pos)
            .into_iter()
            .map(|to| {
                let piece = board.get_piece(to);
                if piece.is_none() {
                    return Move::new(pos, to, None, None);
                } else {
                    return Move::new(pos, to, None, Some((piece.unwrap(), to)));
                }
            })
            .collect::<Vec<Move>>();
        let mut legal_moves = Vec::new();
        for move_ in moves.iter_mut() {
            let color = board.get_piece(move_.from).unwrap().get_color();
            let mut temp_board = board.clone();
            temp_board.force_apply_move(&move_);
            let attacked_squares = temp_board.get_attacked_squares(color.flip());
            let king_pos = temp_board.find_king(color);
            if let Some(king_pos) = king_pos {
                if attacked_squares[king_pos.0 as usize][king_pos.1 as usize] == false {
                    let attacking_squares = temp_board.get_attacked_squares(color);
                    let opp_king_pos = temp_board.find_king(color.flip());
                    if let Some(opp_king_pos) = opp_king_pos {
                        if attacking_squares[opp_king_pos.0 as usize][opp_king_pos.1 as usize] {
                            move_.check = true;
                        }
                    }
                    legal_moves.push(move_.clone());
                }
            }
        }
        legal_moves
    }
}
