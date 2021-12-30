use crate::game::{ Game };
use crate::piece::{ Piece };
use crate::consts::{ Color, PieceType };
use crate::move_struct::{ Move };


#[test]
fn basic_move_test() {
    let mve = Move::simple_new([4, 1], [4, 3]);
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    game.do_move(&mve);

    assert_eq!(game.board[3][4].unwrap().piece_type, PieceType::Pawn);
    assert!(game.board[1][4].is_none());
}

#[test]
fn castle_move_test() {
    // queens side
    let mve = Move { from: [4, 0], to: [2, 0], piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) };
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1"));
    game.do_move(&mve);

    assert!(game.board[0][0].is_none());
    assert!(game.board[0][4].is_none());
    assert!(game.board[0][1].is_none());
    assert_eq!(game.board[0][2].unwrap().piece_type, PieceType::King);
    assert_eq!(game.board[0][3].unwrap().piece_type, PieceType::Rook);

    // kings side
    let mve = Move { from: [4, 0], to: [6, 0], piece: Some(Piece { color: Color::White, piece_type: PieceType::King }) };
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1"));
    game.do_move(&mve);

    assert!(game.board[0][7].is_none());
    assert!(game.board[0][4].is_none());
    assert_eq!(game.board[0][6].unwrap().piece_type, PieceType::King);
    assert_eq!(game.board[0][5].unwrap().piece_type, PieceType::Rook);
}

#[test]
fn promote_move_test() {
    for piece_type in vec![PieceType::Queen, PieceType::Knight, PieceType::Bishop, PieceType::Rook] {
        let mve = Move { from: [0, 6], to: [0, 7], piece: Some(Piece { color: Color::White, piece_type: piece_type }) };
        let mut game = Game::from_fen(String::from("8/P7/8/8/8/8/8/K1k5 w KQkq - 0 1"));
        game.do_move(&mve);
    
        assert_eq!(game.board[7][0].unwrap().piece_type, piece_type);
        assert!(game.board[6][0].is_none());
    }
}

#[test]
fn en_passant_test() {
    let mve = Move { from: [1, 5], to: [0, 6], piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) };
    let mut game = Game::from_fen(String::from("8/8/pP6/8/8/8/8/K1k5 w KQkq a7 0 1"));
    game.do_move(&mve);

    assert!(game.board[0][5].is_none());
    assert_eq!(game.board[6][0].unwrap().piece_type, PieceType::Pawn);
}
