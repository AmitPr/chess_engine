use crate::{Board, Color, Piece};

#[derive(Clone)]
pub struct Move {
    pub from: (i8, i8),
    pub to: (i8, i8),
    pub promotion: Option<Piece>,
    pub captured: Option<(Piece, (i8, i8))>,
    pub check_pieces: Vec<(i8, i8)>,
}

impl Move {
    pub fn new(
        from: (i8, i8),
        to: (i8, i8),
        promotion: Option<Piece>,
        captured: Option<(Piece, (i8, i8))>,
    ) -> Move {
        Move {
            from,
            to,
            promotion,
            captured,
            check_pieces: vec![],
        }
    }

    pub fn move_color(&self, board:Board) -> Option<Color> {
        let piece = board.get_piece(self.from);
        if let Some(piece) = piece {
            Some(piece.get_color())
        } else {
            None
        }
    }

    pub fn is_valid(&self, board: Board) -> bool {
        // Filter out moves that don't change position
        if self.from == self.to {
            return false;
        }

        // Filter out moves that don't have valid positions
        if !((0..9).contains(&self.from.0)
            && (0..9).contains(&self.from.1)
            && (0..9).contains(&self.to.0)
            && (0..9).contains(&self.to.1))
        {
            return false;
        }

        //Filter out moves that don't have valid moving piece
        let from_piece = board.get_piece(self.from);
        if !from_piece.is_some() {
            return false;
        }
        let from_piece = from_piece.unwrap();

        // Filter out moves that don't have valid capture rules
        let piece = board.get_piece(self.to);
        if piece.is_some() {
            if self.captured.is_none() {
                return false;
            }
            let piece = piece.unwrap();
            if piece.get_color() == from_piece.get_color() {
                return false;
            }
            let (capped, _) = self.captured.unwrap();
            if piece != capped{
                return false;
            }
        }

        //Check if piece can actually move to the destination
        let moves = board.get_pseudo_legal_moves(self.from);
        if !moves.contains(&self.to) {
            return false;
        }

        return true;
    }
}
