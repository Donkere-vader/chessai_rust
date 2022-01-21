use crate::game::{ Game };
use crate::piece::{ Piece };
use crate::consts::{ Color, PieceType };
use crate::openings::{ OpeningsDatabase };
use crate::move_struct::{ Move };
use crate::utils::{ string_square_to_square };

#[test]
fn start_game_score_0() {
    // The score for white should be 0 in the starting position (because the scores for black and white should be exactly the same)
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    game.calculate_board_score();
    assert_eq!(game.score_white, 0);
}

#[test]
fn black_more_pieces_white_better_score() {
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w KQkq - 0 1"));
    game.calculate_board_score();
    assert!(game.score_white < 0);
}

#[test]
fn score_predict_same_as_calculate() {
    let mut game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    let moves = vec![
        Move::simple_new([1, 0], [0, 2]),
        Move::simple_new([1, 7], [0, 5]),
        Move::simple_new([3, 1], [3, 3]),
        Move::simple_new([3, 6], [3, 4]),
        Move::simple_new([2, 0], [4, 2]),
        Move::simple_new([2, 7], [4, 5]),
        Move::simple_new([3, 0], [3, 1]),
        Move::simple_new([3, 7], [3, 6]),
        Move { from: [4, 0], to: [0, 0], piece: Some(Piece { piece_type: PieceType::Queen, color: Color::White}) },
        Move::simple_new([3, 6], [4, 0]),
    ];

    for mve in moves.iter() {
        game.do_move(mve);
        let predicted_score = game.score_white;
        game.calculate_board_score();

        assert_eq!(predicted_score, game.score_white);

    }

    game.show_board(None, Color::White);
}

#[test]
fn dont_check_self() {
    let game = Game::from_fen(String::from("kr6/r7/8/8/8/8/8/2K5 w KQkq - 0 100"));
    let opening_database = OpeningsDatabase::new();
    let best_move = game.get_best_move(6, &opening_database);

    assert_ne!(best_move.to[0], 1);
}

#[test]
fn max_score_without_black_king() {
    let mut game = Game::from_fen(String::from("rnbq1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    game.calculate_board_score();
    assert_eq!(game.score_white, i64::MAX);
}


#[test]
fn piece_from_fen() {
    let new_piece = Piece::from_fen('Q');

    assert_eq!(new_piece.color, Color::White);
    assert_eq!(new_piece.piece_type, PieceType::Queen);
}

#[test]
fn finds_opening_move() {
    let opening_database = OpeningsDatabase::new();
    let mve = opening_database.find_opening(&vec![Move::simple_new([4, 1], [4, 3])]);
    assert!(mve.is_some());
}

#[test]
fn string_square_to_square_test() {
    assert_eq!(string_square_to_square(String::from("a1")), [0, 0]);
    assert_eq!(string_square_to_square(String::from("e4")), [4, 3]);
    assert_eq!(string_square_to_square(String::from("b5")), [1, 4]);
    assert_eq!(string_square_to_square(String::from("f5")), [5, 4]);
}

// #[test]
// fn game_phase_calculator() {
//     let fen_codes: Vec<(String, GamePhase)> = vec![
//         ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(), GamePhase::StartGame),
//         ("r2qkbnr/2pNpppp/p1Q5/8/3p2b1/2P5/PP1PPP1P/RNB1KB1R w KQkq - 0 9".to_string(), GamePhase::MidGame),
//         ("7k/7r/5r2/K7/8/8/8/8 b KQkq - 0 1".to_string(), GamePhase::EndGame),
//     ];

//     for fen_code_combi in fen_codes.iter() {
//         let mut new_game = Game::from_fen(fen_code_combi.0.to_string());
//         new_game.calculate_game_phase();
//         println!("{}", fen_code_combi.0);
//         new_game.show_board(None, Color::White);
//         assert_eq!(fen_code_combi.1, new_game.game_phase);
//     }
// }
