use crate::game::{ Game };
use crate::piece::{ Piece };
use crate::consts::{ Color, PieceType, MoveType };
use crate::move_struct::{ Move };


#[test]
fn move_recognized_as_castle() {
    let mve = Move { from: [4, 0], to: [2, 0], piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) };
    let game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1"));
    let (mve_type, _mve_piece) = mve.get_move_type(Some(&game.castle), game.en_passant_target_square, Some(game.board[0][4].unwrap().piece_type));
    assert_eq!(mve_type, MoveType::Castle);
}

#[test]
fn move_recognized_as_promote() {
    let mve = Move { from: [0, 6], to: [0, 7], piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) };
    let game = Game::from_fen(String::from("8/P7/8/8/8/8/8/K1k5 w KQkq - 0 1"));
    let (mve_type, _mve_piece) = mve.get_move_type(Some(&game.castle), game.en_passant_target_square, Some(game.board[6][0].unwrap().piece_type));
    assert_eq!(mve_type, MoveType::Promote);
}

#[test]
fn move_recognized_as_en_passant() {
    let mve = Move { from: [1, 5], to: [0, 6], piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) };
    let game = Game::from_fen(String::from("8/8/pP6/8/8/8/8/K1k5 w KQkq a7 0 1"));
    let (mve_type, _mve_piece) = mve.get_move_type(Some(&game.castle), game.en_passant_target_square, Some(game.board[5][1].unwrap().piece_type));
    assert_eq!(mve_type, MoveType::EnPassant);
}
