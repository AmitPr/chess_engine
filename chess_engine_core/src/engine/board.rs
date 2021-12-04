use crate::{
    logic::{bishop, queen, rook},
    Color, Move, Piece,
};

use super::castling::Castling;
#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub en_passant: [bool; 16],
    pub castling: Castling,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            pieces: [[None; 8]; 8],
            turn: Color::White,
            en_passant: [false; 16],
            castling: Castling::new(),
        }
    }
    pub fn new() -> Board {
        let mut pieces = [[None; 8]; 8];
        for i in 0..8 {
            pieces[1][i] = Some(Piece::Pawn(Color::White));
            pieces[6][i] = Some(Piece::Pawn(Color::Black));
        }

        for i in [0, 7].iter() {
            let row = *i;
            let col = if row == 0 { Color::White } else { Color::Black };
            pieces[row][0] = Some(Piece::Rook(col.clone()));
            pieces[row][1] = Some(Piece::Knight(col.clone()));
            pieces[row][2] = Some(Piece::Bishop(col.clone()));
            pieces[row][3] = Some(Piece::Queen(col.clone()));
            pieces[row][4] = Some(Piece::King(col.clone()));
            pieces[row][5] = Some(Piece::Bishop(col.clone()));
            pieces[row][6] = Some(Piece::Knight(col.clone()));
            pieces[row][7] = Some(Piece::Rook(col.clone()));
        }

        Board {
            pieces,
            turn: Color::White,
            en_passant: [false; 16],
            castling: Castling::new(),
        }
    }

    pub fn get_piece(&self, pos: (i8, i8)) -> Option<Piece> {
        assert!(pos.0 >= 0 && pos.0 < 8);
        assert!(pos.1 >= 0 && pos.1 < 8);
        self.pieces[pos.0 as usize][pos.1 as usize].clone()
    }

    pub fn get_piece_at(&self, row: i8, col: i8) -> Option<Piece> {
        assert!(row >= 0 && row < 8);
        assert!(col >= 0 && col < 8);
        self.pieces[row as usize][col as usize].clone()
    }

    pub fn get_pseudo_legal_moves(&self, pos: (i8, i8)) -> Vec<(i8, i8)> {
        let piece = self.get_piece(pos);
        if piece.is_none() {
            return vec![];
        }
        let piece = piece.unwrap();
        match piece {
            Piece::Pawn(color) => return crate::logic::pawn::logic(*self, pos, color),
            Piece::Knight(color) => return crate::logic::knight::logic(*self, pos, color),
            Piece::Bishop(color) => return crate::logic::bishop::logic(*self, pos, color),
            Piece::Rook(color) => return crate::logic::rook::logic(*self, pos, color),
            Piece::Queen(color) => return crate::logic::queen::logic(*self, pos, color),
            Piece::King(color) => return crate::logic::king::logic(*self, pos, color),
        }
    }

    pub fn get_legal_moves(&self, pos: (i8, i8)) -> Vec<Move> {
        // Gets all pseudo legal moves
        let mut moves = self
            .get_pseudo_legal_moves(pos)
            .into_iter()
            .map(|to| {
                let piece = self.get_piece(to);
                if piece.is_none() {
                    return Move::new(pos, to, None, None);
                } else {
                    return Move::new(pos, to, None, Some((piece.unwrap(), to)));
                }
            })
            .collect::<Vec<Move>>();
        let mut legal_moves = Vec::new();
        // Check if each move is legal by applying it on a copy of the board
        for move_ in moves.iter_mut() {
            let color = self.get_piece(move_.from).unwrap().get_color();

            let mut temp_board = self.clone();
            temp_board.force_apply_move(&move_);

            let attacked_squares = temp_board.get_attacked_squares(color.flip(), false);
            let king_pos = temp_board.find_king(color);

            if let Some(king_pos) = king_pos {
                // Check if the king's square is being attacked by zero pieces
                if attacked_squares[king_pos.0 as usize][king_pos.1 as usize].len() == 0 {
                    legal_moves.push(move_.clone());
                }
            }
        }
        legal_moves
    }

    /// Returns a 2D Vector of all the positions of the piece that attack the tile at (i,j)
    pub fn get_attacked_squares(
        self,
        attacker_color: Color,
        use_legal: bool,
    ) -> Vec<Vec<Vec<(i8, i8)>>> {
        let mut attacked_squares = vec![vec![vec![]; 8]; 8];
        for row in 0..8 {
            for col in 0..8 {
                let piece = self.get_piece_at(row, col);
                if let Some(piece) = piece {
                    if piece.get_color() != attacker_color {
                        continue;
                    }
                    if use_legal {
                        let moves = self.get_legal_moves((row, col));
                        for move_ in moves.iter() {
                            attacked_squares[move_.to.0 as usize][move_.to.1 as usize]
                                .push(move_.from);
                        }
                    } else {
                        for to in self.get_pseudo_legal_moves((row, col)).iter() {
                            attacked_squares[to.0 as usize][to.1 as usize].push((row, col));
                        }
                    }
                }
            }
        }
        attacked_squares
    }

    pub fn find_king(&self, color: Color) -> Option<(i8, i8)> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.get_piece_at(row, col) {
                    match piece {
                        Piece::King(c) => {
                            if c == color {
                                return Some((row, col));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Applies a move without checking any legality.
    pub fn force_apply_move(&mut self, move_: &Move) {
        let from = move_.from;
        let to = move_.to;
        if let Some((_captured, pos)) = move_.captured {
            self.pieces[pos.0 as usize][pos.1 as usize] = None;
        }
        let piece = self.get_piece(from);
        self.pieces[from.0 as usize][from.1 as usize] = None;
        self.pieces[to.0 as usize][to.1 as usize] = piece;

        let piece = piece.unwrap();
        //Update En Passant tracker
        match piece {
            Piece::Pawn(color) => {
                let dist = (to.0 - from.0).abs();
                if color == Color::White {
                    if to.0 == 3 {
                        if dist == 2 {
                            self.en_passant[to.1 as usize] = true;
                        } else {
                            self.en_passant[to.1 as usize] = false;
                        }
                    }
                    if from.0 == 3 {
                        self.en_passant[from.1 as usize] = false;
                    }
                } else {
                    if to.0 == 4 {
                        if dist == 2 {
                            self.en_passant[to.1 as usize + 8] = true;
                        } else {
                            self.en_passant[to.1 as usize + 8] = false;
                        }
                    }
                    if from.0 == 4 {
                        self.en_passant[from.1 as usize + 8] = false;
                    }
                }
            }
            _ => {}
        };
    }

    /// Precondition: the move is legal.
    /// Applies a move to this board.
    ///
    /// Returns true if the move was a checkmate.
    pub fn apply_move(&mut self, move_: &Move) -> bool {
        let color = move_.move_color(*self).unwrap();
        let opponent_color = color.flip();
        self.force_apply_move(move_);

        // Check if the opponent's king is now in check (and by how many pieces)
        let attacking_squares = self.get_attacked_squares(color, false);
        let opp_king_pos = self.find_king(opponent_color);
        let mut check_pieces: Vec<(i8, i8)> = vec![];
        if let Some(opp_king_pos) = opp_king_pos {
            let attacking_pieces =
                &attacking_squares[opp_king_pos.0 as usize][opp_king_pos.1 as usize];
            if attacking_pieces.len() > 0 {
                check_pieces = attacking_pieces.clone();
            }
        }

        if check_pieces.len() == 0 {
            return false;
        }
        let king_pos = self.find_king(opponent_color).unwrap();
        let king_moves = self.get_legal_moves(king_pos);
        // Check if the king can move itself out of check
        if king_moves.len() > 0 {
            return false;
        }
        //If there are two pieces giving check, the king must move to avoid checkmate.
        if check_pieces.len() > 1 {
            return true;
        }

        //At this point, we need to check if the piece giving check can be captured, or blocked.
        let attacker_pos = check_pieces[0];
        let counter_squares = self.get_attacked_squares(opponent_color, true);
        for row in 0..8 {
            for col in 0..8 {
                if counter_squares[row][col].contains(&attacker_pos) {
                    return false;
                }
            }
        }
        //The king can't move, the piece giving check can't be captured, so it must be blocked.
        let attacker_piece = self.get_piece(attacker_pos).unwrap();
        //Get the ray along which the attacker is giving check.
        let attacker_ray = match attacker_piece {
            Piece::Bishop(_) => bishop::get_ray_to_king(*self, attacker_pos, opponent_color.flip()),
            Piece::Queen(_) => queen::get_ray_to_king(*self, attacker_pos, opponent_color.flip()),
            Piece::Rook(_) => rook::get_ray_to_king(*self, attacker_pos, opponent_color.flip()),
            _ => Vec::<(i8, i8)>::new(),
        };
        //Checks if the piece giving check can be blocked by seeing if there are legal moves
        //any of the squares in the ray casted towards the king.
        for (x, y) in attacker_ray {
            //Check if the ray at this square can potentially be moved to.
            if counter_squares[x as usize][y as usize].len() > 0 {
                let blockers = &counter_squares[x as usize][y as usize];
                //Check if any of the pieces that could move to block can do so legally.
                for b_pos in blockers {
                    let copy = self.clone();
                    let blocker_moves = copy.get_legal_moves(*b_pos);
                    for blocker_move in blocker_moves {
                        if blocker_move.to == (x, y) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
