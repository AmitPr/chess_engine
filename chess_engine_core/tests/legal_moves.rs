use chess_engine_core::{self, Board, Color, Piece};

#[test]
fn test_legal_no_check(){
    let mut board = Board::empty();
    board.pieces[0][0] = Some(Piece::King(Color::White));
    board.pieces[1][1] = Some(Piece::Pawn(Color::White));
    let moves = board.get_legal_moves((1,1));
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_legal_dont_expose_king(){
    let mut board = Board::empty();
    board.pieces[0][0] = Some(Piece::King(Color::White));
    board.pieces[1][1] = Some(Piece::Pawn(Color::White));
    board.pieces[7][7] = Some(Piece::Bishop(Color::Black));
    let moves = board.get_legal_moves((1,1));
    assert_eq!(moves.len(), 0);
}