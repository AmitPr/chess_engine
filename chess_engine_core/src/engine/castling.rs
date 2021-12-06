#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Castling {
    pub white: [bool; 3],
    pub black: [bool; 3],
}

impl Castling {
    pub fn new() -> Castling {
        Castling {
            white: [true, true, true],
            black: [true, true, true],
        }
    }
}