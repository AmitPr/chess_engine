use chess_engine_core::{Board, Color, Move, Piece};
use js_sys::Array;
use wasm_bindgen::prelude::*;

//Build:
//wasm-pack build --target web chess_engine_wasm_bindings && cp -r chess_engine_wasm_bindings/pkg site/static/
#[wasm_bindgen]
pub struct JsBoard {
    inner: Board,
}
#[wasm_bindgen]
impl JsBoard {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsBoard {
        let board = Board::new();
        JsBoard { inner: board }
    }

    #[wasm_bindgen]
    pub fn get_piece_at(&self, row: usize, col: usize) -> Option<JsPiece> {
        let piece = self.inner.pieces[row][col];
        match piece {
            Some(piece) => Some(JsPiece::new(&piece)),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> Array {
        let pieces = self.inner.pieces;
        let mut result = Vec::new();
        for row in pieces.iter() {
            let mut row_vec = Vec::new();
            for piece in row.iter() {
                row_vec.push(piece.map(|p| JsPiece::new(&p)));
            }
            result.push(row_vec.into_iter().map(JsValue::from).collect::<Array>());
        }
        result.into_iter().map(JsValue::from).collect()
    }

    pub fn get_legal_moves(&self, row: i8, col: i8) -> Array {
        let moves = self.inner.get_legal_moves((row, col));
        let mut result = Vec::new();
        for move_ in moves {
            result.push(JsMove::from_move(move_));
        }
        result.into_iter().map(JsValue::from).collect()
    }

    pub fn apply_move(&mut self, move_: JsMove) -> bool {
        let move_ = move_.get();
        self.inner.apply_move(&move_)
    }

    pub fn get_move_from_pos(&self, from_row: i8, from_col: i8, to_row: i8, to_col: i8) -> Option<JsMove> {
        let moves = self.inner.get_legal_moves((from_row, from_col));
        for move_ in moves {
            if move_.to == (to_row, to_col) {
                return Some(JsMove::from_move(move_));
            }
        }
        None
    }
}

#[wasm_bindgen]
pub struct JsPiece {
    inner: Piece,
}

#[wasm_bindgen]
impl JsPiece {
    pub fn is_white(&self) -> bool {
        self.inner.get_color() == Color::White
    }
    pub fn is_black(&self) -> bool {
        self.inner.get_color() == Color::Black
    }

    pub fn get_piece_type(&self) -> String {
        match self.inner {
            Piece::Pawn(_) => "pawn".to_string(),
            Piece::Rook(_) => "rook".to_string(),
            Piece::Knight(_) => "knight".to_string(),
            Piece::Bishop(_) => "bishop".to_string(),
            Piece::Queen(_) => "queen".to_string(),
            Piece::King(_) => "king".to_string(),
        }
    }

    pub fn get_color(&self) -> String {
        match self.inner.get_color() {
            Color::White => "white".to_string(),
            Color::Black => "black".to_string(),
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        match self.inner {
            Piece::Pawn(Color::White) => "Pawn(White)".to_string(),
            Piece::Pawn(Color::Black) => "Pawn(Black)".to_string(),
            Piece::Rook(Color::White) => "Rook(White)".to_string(),
            Piece::Rook(Color::Black) => "Rook(Black)".to_string(),
            Piece::Knight(Color::White) => "Knight(White)".to_string(),
            Piece::Knight(Color::Black) => "Knight(Black)".to_string(),
            Piece::Bishop(Color::White) => "Bishop(White)".to_string(),
            Piece::Bishop(Color::Black) => "Bishop(Black)".to_string(),
            Piece::Queen(Color::White) => "Queen(White)".to_string(),
            Piece::Queen(Color::Black) => "Queen(Black)".to_string(),
            Piece::King(Color::White) => "King(White)".to_string(),
            Piece::King(Color::Black) => "King(Black)".to_string(),
        }
    }
}

impl JsPiece {
    pub fn new(piece: &Piece) -> JsPiece {
        JsPiece {
            inner: piece.clone(),
        }
    }
    pub fn get(&self) -> Piece {
        self.inner.clone()
    }
}

#[wasm_bindgen]
pub struct JsMove {
    inner: Move,
    from: pos,
    to: pos,
}

#[wasm_bindgen]
impl JsMove {
    #[wasm_bindgen(getter)]
    pub fn from(&self) -> pos {
        let pos = self.inner.from;
        pos { x: pos.0, y: pos.1 }
    }
    #[wasm_bindgen(setter)]
    pub fn set_from(&mut self, from: pos) {
        self.inner.from = (from.x, from.y);
        self.from = from;
    }

    #[wasm_bindgen(getter)]
    pub fn to(&self) -> pos {
        let pos = self.inner.to;
        pos { x: pos.0, y: pos.1 }
    }
    #[wasm_bindgen(setter)]
    pub fn set_to(&mut self, to: pos) {
        self.inner.to = (to.x, to.y);
        self.to = to;
    }
}

impl JsMove {
    pub fn from_move(move_: Move) -> JsMove {
        JsMove {
            inner: move_,
            from: pos {
                x: move_.from.0,
                y: move_.from.1,
            },
            to: pos {
                x: move_.to.0,
                y: move_.to.1,
            },
        }
    }

    pub fn get(&self) -> Move {
        self.inner.clone()
    }
}

#[wasm_bindgen(inspectable)]
pub struct pos {
    pub x: i8,
    pub y: i8,
}
