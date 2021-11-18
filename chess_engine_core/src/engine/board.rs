use super::{castling::Castling, color::Color, piece::Piece};
#[derive(Clone, Copy)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub en_passant: Option<(usize, usize)>,
    pub castling: Castling,
}

impl Board {
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
            en_passant: None,
            castling: Castling::new(),
        }
    }

    pub fn get_piece(&self, pos: (i8, i8)) -> Option<Piece> {
        self.pieces[pos.0 as usize][pos.1 as usize].clone()
    }

    pub fn get_piece_at(&self, row: i8, col: i8) -> Option<Piece> {
        self.pieces[row as usize][col as usize].clone()
    }

    /*pub fn apply_move(self, move_: Move) -> Board {
        let mut board = self.clone();
        board
     }*/
}
