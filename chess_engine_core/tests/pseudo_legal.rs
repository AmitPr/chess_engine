use chess_engine_core::{self, Board, Color, Piece};

/// Tests the `get_pseudo_legal_moves` function.
///
/// This means that all the piece logic is tested in this file.
/// Queen Logic is omitted, as it is tested by bishop and rook.

#[test]
fn test_pseudo_legal_pawn_moves() {
    let mut board = Board::empty();
    // Expect white pawn moving forward (1 and 2 squares expected)
    board.pieces[1][0] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((1, 0));
    assert_eq!(moves.len(), 2);
    assert_eq!(moves[0], (2, 0));
    assert_eq!(moves[1], (3, 0));
    // Expect black pawn moving forward (1 and 2 squares expected)
    board = Board::empty();
    board.pieces[6][0] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((6, 0));
    assert_eq!(moves.len(), 2);
    assert_eq!(moves[0], (5, 0));
    assert_eq!(moves[1], (4, 0));
    // Expect blocked pawn not to move
    board = Board::empty();
    board.pieces[1][0] = Some(Piece::Pawn(Color::White));
    board.pieces[2][0] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((1, 1));
    assert_eq!(moves.len(), 0);
    // Expect pawn to only move one square if not on initial row
    board = Board::empty();
    board.pieces[2][0] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((2, 0));
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], (3, 0));
    // Expect pawn to be able to capture diagonally
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Pawn(Color::White));
    board.pieces[4][5] = Some(Piece::Pawn(Color::White));
    board.pieces[5][4] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][5] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], (5, 5));
    let moves = board.get_pseudo_legal_moves((5, 5));
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], (4, 4));
    // Expect pawn not to be able to go off board
    board = Board::empty();
    board.pieces[0][0] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((0, 0));
    assert_eq!(moves.len(), 0);
    //TODO: test en passant, pawn promotion
}

#[test]
fn test_pseudo_legal_rook_moves() {
    let mut board = Board::empty();
    // Expect rook to move in all directions
    board.pieces[4][4] = Some(Piece::Rook(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 14);
    assert!(moves.contains(&(4, 5)));
    assert!(moves.contains(&(4, 6)));
    assert!(moves.contains(&(4, 7)));
    assert!(moves.contains(&(4, 0)));
    assert!(moves.contains(&(4, 1)));
    assert!(moves.contains(&(4, 2)));
    assert!(moves.contains(&(4, 3)));
    assert!(moves.contains(&(5, 4)));
    assert!(moves.contains(&(6, 4)));
    assert!(moves.contains(&(7, 4)));
    assert!(moves.contains(&(0, 4)));
    assert!(moves.contains(&(1, 4)));
    assert!(moves.contains(&(2, 4)));
    assert!(moves.contains(&(3, 4)));
    // Expect rook to be blocked by another piece
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Rook(Color::White));

    board.pieces[4][5] = Some(Piece::Pawn(Color::White));
    board.pieces[4][3] = Some(Piece::Pawn(Color::White));
    board.pieces[5][4] = Some(Piece::Pawn(Color::White));
    board.pieces[3][4] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 0);

    // Expect rook to be able to capture capture on lines
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Rook(Color::White));

    board.pieces[4][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[4][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][4] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][4] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][5] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 4);
    assert!(moves.contains(&(4, 5)));
    assert!(moves.contains(&(4, 3)));
    assert!(moves.contains(&(5, 4)));
    assert!(moves.contains(&(3, 4)));
}

#[test]
fn test_pseudo_legal_knight_moves() {
    let mut board = Board::empty();
    // Expect knight to move in all L-shaped squares
    board.pieces[4][4] = Some(Piece::Knight(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 8);
    assert!(moves.contains(&(2, 3)));
    assert!(moves.contains(&(2, 5)));
    assert!(moves.contains(&(3, 2)));
    assert!(moves.contains(&(3, 6)));
    assert!(moves.contains(&(5, 2)));
    assert!(moves.contains(&(5, 6)));
    assert!(moves.contains(&(6, 3)));
    assert!(moves.contains(&(6, 5)));
    // Expect knight not to be blocked by adjacent pieces
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Knight(Color::White));
    board.pieces[4][5] = Some(Piece::Pawn(Color::White));
    board.pieces[4][3] = Some(Piece::Pawn(Color::White));
    board.pieces[5][4] = Some(Piece::Pawn(Color::White));
    board.pieces[3][4] = Some(Piece::Pawn(Color::White));
    board.pieces[5][5] = Some(Piece::Pawn(Color::White));
    board.pieces[3][3] = Some(Piece::Pawn(Color::White));
    board.pieces[5][3] = Some(Piece::Pawn(Color::White));
    board.pieces[3][5] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 8);
    assert!(moves.contains(&(2, 3)));
    assert!(moves.contains(&(2, 5)));
    assert!(moves.contains(&(3, 2)));
    assert!(moves.contains(&(3, 6)));
    assert!(moves.contains(&(5, 2)));
    assert!(moves.contains(&(5, 6)));
    assert!(moves.contains(&(6, 3)));
    assert!(moves.contains(&(6, 5)));

    // Expect knight to be blocked by pieces at L-shaped squares
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Knight(Color::White));
    board.pieces[2][3] = Some(Piece::Pawn(Color::White));
    board.pieces[2][5] = Some(Piece::Pawn(Color::White));
    board.pieces[3][2] = Some(Piece::Pawn(Color::White));
    board.pieces[3][6] = Some(Piece::Pawn(Color::White));
    board.pieces[5][2] = Some(Piece::Pawn(Color::White));
    board.pieces[5][6] = Some(Piece::Pawn(Color::White));
    board.pieces[6][3] = Some(Piece::Pawn(Color::White));
    board.pieces[6][5] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 0);

    // Expect knight to be able to capture pieces at L-shaped squares
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Knight(Color::White));
    board.pieces[2][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[2][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][2] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][6] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][2] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][6] = Some(Piece::Pawn(Color::Black));
    board.pieces[6][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[6][5] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 8);

    // Expect knight not to move out of bounds
    board = Board::empty();
    board.pieces[0][0] = Some(Piece::Knight(Color::White));
    let moves = board.get_pseudo_legal_moves((0, 0));
    assert_eq!(moves.len(), 2);
}

#[test]
fn test_pseudo_legal_bishop_moves() {
    let mut board = Board::empty();
    // Expect bishop to move diagonally
    board.pieces[4][4] = Some(Piece::Bishop(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 13);
    assert!(moves.contains(&(0, 0)));
    assert!(moves.contains(&(1, 1)));
    assert!(moves.contains(&(2, 2)));
    assert!(moves.contains(&(3, 3)));
    assert!(moves.contains(&(5, 5)));
    assert!(moves.contains(&(6, 6)));
    assert!(moves.contains(&(7, 7)));
    assert!(moves.contains(&(7, 1)));
    assert!(moves.contains(&(6, 2)));
    assert!(moves.contains(&(5, 3)));
    assert!(moves.contains(&(3, 5)));
    assert!(moves.contains(&(2, 6)));
    assert!(moves.contains(&(1, 7)));

    // Expect bishop to be blocked by pieces
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Bishop(Color::White));
    board.pieces[3][3] = Some(Piece::Pawn(Color::White));
    board.pieces[5][5] = Some(Piece::Pawn(Color::White));
    board.pieces[3][5] = Some(Piece::Pawn(Color::White));
    board.pieces[5][3] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 0);

    // Expect bishop to be able to capture pieces
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::Bishop(Color::White));
    board.pieces[3][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][3] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 4);
    assert!(moves.contains(&(3, 3)));
    assert!(moves.contains(&(5, 5)));
    assert!(moves.contains(&(3, 5)));
    assert!(moves.contains(&(5, 3)));
}

#[test]
fn test_pseudo_legal_king_moves() {
    let mut board = Board::empty();
    // Expect king to move one square in any direction
    board.pieces[4][4] = Some(Piece::King(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 8);
    assert!(moves.contains(&(3, 3)));
    assert!(moves.contains(&(5, 5)));
    assert!(moves.contains(&(3, 5)));
    assert!(moves.contains(&(5, 3)));
    assert!(moves.contains(&(3, 4)));
    assert!(moves.contains(&(5, 4)));
    assert!(moves.contains(&(4, 3)));
    assert!(moves.contains(&(4, 5)));

    // Expect king to be blocked by pieces
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::King(Color::White));
    board.pieces[4][3] = Some(Piece::Pawn(Color::White));
    board.pieces[4][5] = Some(Piece::Pawn(Color::White));
    board.pieces[3][4] = Some(Piece::Pawn(Color::White));
    board.pieces[5][4] = Some(Piece::Pawn(Color::White));
    board.pieces[3][3] = Some(Piece::Pawn(Color::White));
    board.pieces[5][5] = Some(Piece::Pawn(Color::White));
    board.pieces[3][5] = Some(Piece::Pawn(Color::White));
    board.pieces[5][3] = Some(Piece::Pawn(Color::White));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 0);

    // Expect king to be able to capture pieces
    board = Board::empty();
    board.pieces[4][4] = Some(Piece::King(Color::White));
    board.pieces[4][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[4][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][4] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][4] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][3] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[3][5] = Some(Piece::Pawn(Color::Black));
    board.pieces[5][3] = Some(Piece::Pawn(Color::Black));
    let moves = board.get_pseudo_legal_moves((4, 4));
    assert_eq!(moves.len(), 8);
    assert!(moves.contains(&(3, 3)));
    assert!(moves.contains(&(5, 5)));
    assert!(moves.contains(&(3, 5)));
    assert!(moves.contains(&(5, 3)));
    assert!(moves.contains(&(3, 4)));
    assert!(moves.contains(&(5, 4)));
    assert!(moves.contains(&(4, 3)));
    assert!(moves.contains(&(4, 5)));
}
