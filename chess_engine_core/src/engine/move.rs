use crate::Piece;

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promotion: Option<Piece>,
    pub captured: (Option<Piece>, Option<(usize, usize)>),
}

impl Move {
    pub fn new(
        from: (usize, usize),
        to: (usize, usize),
        promotion: Option<Piece>,
        captured: (Option<Piece>, Option<(usize, usize)>),
    ) -> Move {
        Move {
            from,
            to,
            promotion,
            captured,
        }
    }
}
