#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Castling {
    pub white: [bool; 4],
    pub black: [bool; 4],
}

impl Castling {
    pub fn new() -> Castling {
        Castling {
            white: [true, true, true, true],
            black: [true, true, true, true],
        }
    }
}