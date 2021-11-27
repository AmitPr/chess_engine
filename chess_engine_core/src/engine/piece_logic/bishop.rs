use crate::{Board, Color, Piece};

pub fn logic(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let x = pos.0;
    let y = pos.1;
    let mut i = 1;
    while x + i < 8 && y + i < 8 {
        if board.get_piece((x + i, y + i)).is_none() {
            moves.push((x + i, y + i));
        } else if board.get_piece((x + i, y + i)).unwrap().get_color() != color {
            moves.push((x + i, y + i));
            break;
        } else {
            break;
        }
        i += 1;
    }
    i = 1;
    while x - i >= 0 && y + i < 8 {
        if board.get_piece((x - i, y + i)).is_none() {
            moves.push((x - i, y + i));
        } else if board.get_piece((x - i, y + i)).unwrap().get_color() != color {
            moves.push((x - i, y + i));
            break;
        } else {
            break;
        }
        i += 1;
    }
    i = 1;
    while x + i < 8 && y - i >= 0 {
        if board.get_piece((x + i, y - i)).is_none() {
            moves.push((x + i, y - i));
        } else if board.get_piece((x + i, y - i)).unwrap().get_color() != color {
            moves.push((x + i, y - i));
            break;
        } else {
            break;
        }
        i += 1;
    }
    i = 1;
    while x - i < 8 && y - i >= 0 {
        if board.get_piece((x - i, y - i)).is_none() {
            moves.push((x - i, y - i));
        } else if board.get_piece((x - i, y - i)).unwrap().get_color() != color {
            moves.push((x - i, y - i));
            break;
        } else {
            break;
        }
        i += 1;
    }
    moves
}

pub fn get_ray_to_king(board: Board, pos: (i8, i8), color: Color) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    let x = pos.0;
    let y = pos.1;
    let mut i = 1;
    while x + i < 8 && y + i < 8 {
        let piece = board.get_piece((x + i, y + i));
        if piece.is_none() {
            moves.push((x + i, y + i));
        } else if piece.unwrap().get_color() != color
            && piece.unwrap().eq(&Piece::King(color.flip()))
        {
            moves.push((x + i, y + i));
            return moves;
        } else {
            break;
        }
        i += 1;
    }
    moves.clear();
    i = 1;
    while x - i >= 0 && y + i < 8 {
        let piece = board.get_piece((x - i, y + i));
        if piece.is_none() {
            moves.push((x - i, y + i));
        } else if piece.unwrap().get_color() != color
            && piece.unwrap().eq(&Piece::King(color.flip()))
        {
            moves.push((x - i, y + i));
            return moves;
        } else {
            break;
        }
        i += 1;
    }
    moves.clear();
    i = 1;
    while x + i < 8 && y - i >= 0 {
        let piece = board.get_piece((x + i, y - i));
        if piece.is_none() {
            moves.push((x + i, y - i));
        } else if piece.unwrap().get_color() != color
            && piece.unwrap().eq(&Piece::King(color.flip()))
        {
            moves.push((x + i, y - i));
            return moves;
        } else {
            break;
        }
        i += 1;
    }
    i = 1;
    moves.clear();
    while x - i < 8 && y - i >= 0 {
        let piece = board.get_piece((x + i, y - i));
        if piece.is_none() {
            moves.push((x + i, y - i));
        } else if piece.unwrap().get_color() != color
            && piece.unwrap().eq(&Piece::King(color.flip()))
        {
            moves.push((x + i, y - i));
            return moves;
        } else {
            break;
        }
        i += 1;
    }
    moves.clear();
    moves
}
