use chess_engine_core::{self, Board, Color, Move, Piece};

#[test]
fn test_legal_no_check() {
    let mut board = Board::empty();
    board.castling.black = [false, false, false];
    board.castling.white = [false, false, false];
    board.pieces[0][0] = Some(Piece::King(Color::White));
    board.pieces[1][1] = Some(Piece::Pawn(Color::White));
    let moves = board.get_legal_moves((1, 1));
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_legal_dont_expose_king() {
    let mut board = Board::empty();
    board.castling.black = [false, false, false];
    board.castling.white = [false, false, false];
    board.pieces[0][0] = Some(Piece::King(Color::White));
    board.pieces[1][1] = Some(Piece::Pawn(Color::White));
    board.pieces[7][7] = Some(Piece::Bishop(Color::Black));
    let moves = board.get_legal_moves((1, 1));
    assert_eq!(moves.len(), 0);
}

#[test]
fn test_en_passant() {
    let mut board = Board::empty();
    board.pieces[0][0] = Some(Piece::King(Color::White));
    board.pieces[7][7] = Some(Piece::King(Color::Black));
    board.pieces[4][0] = Some(Piece::Pawn(Color::White));
    board.pieces[6][1] = Some(Piece::Pawn(Color::Black));
    let move_ = Move::new((6, 1), (4, 1), None, None);
    board.apply_move(&move_);
    assert!(board.en_passant[9]);
    let moves = board.get_legal_moves((4, 0));
    assert_eq!(moves.len(), 2);
    let move_ = moves[1];
    board.apply_move(&move_);
    assert_eq!(board.pieces[4][0], None);
    assert_eq!(board.pieces[5][1], Some(Piece::Pawn(Color::White)));
    assert_eq!(board.pieces[4][1], None);
}
