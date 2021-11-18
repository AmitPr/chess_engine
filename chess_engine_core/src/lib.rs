mod engine;


pub type Board = crate::engine::board::Board;
pub type Move = crate::engine::r#move::Move;
pub type Castling = crate::engine::castling::Castling;
pub type Piece = crate::engine::piece::Piece;
pub type Color = crate::engine::color::Color;

#[path = "engine/piece_logic/piece_logic.rs"]
pub mod logic;
