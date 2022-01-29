//! The actual functions for the Minimax algorithm.
//! 
//! All functions are in an impl for Game.

use crate::piece::{ get_all_piece_moves };
use crate::consts::{ Color, PieceType, CHECK_MATE_SCORE };
use crate::move_struct::{ Move };
use crate::utils::{ with_offsets };
use std::thread;
use crate::openings::{ OpeningsDatabase };
use rand::Rng;
use crate::game::{ Game };
use crate::types::{ Cord };


impl Game {
    //! Best move implementations

    pub fn square_is_attacked(&self, square: Cord, color: Color) -> bool {
        //! Returns true if the square at the given coordinate is under attack from the specified color.

        let other_color = if color == Color::White { Color::Black } else { Color::White };
        for piece_type in vec![PieceType::Knight, PieceType::Rook, PieceType::Bishop, PieceType::Pawn, PieceType::Queen] {
            for mve in get_all_piece_moves(piece_type, other_color, square, &self).iter() {
                let is_attacked = match self.board[mve.to[1] as usize][mve.to[0] as usize] {
                    Some(p) => { if p.piece_type == piece_type { true } else { false }},
                    None => false,
                };
                if is_attacked { return true };
            }
        }

        for mve in with_offsets(&Color::White, square, self.board, vec![[-1, 0], [1, 0], [-1, -1], [1, -1], [-1, 1], [1, 1], [0, -1], [0, 1]], true).iter() {
            let is_attacked = match self.board[mve.to[1] as usize][mve.to[0] as usize] {
                Some(p) => { if p.piece_type == PieceType::King { true } else { false }},
                None => false,
            };
            if is_attacked { return true };
        }

        false
    }

    pub fn get_best_move(&self, depth: u8, opening_database: &OpeningsDatabase) -> Move {
        //! Returns a move either from the openings database or from the Minimax algorithm
        //! 
        //! If there is a opening to be played it will go for that option.
        //! Otherwise it will spawn threads to calculate the best move using the Minimax algorithm.

        // check for move from opening database
        if self.moves.len() == self.fullmove_counter {
            match opening_database.find_opening(&self.moves) {
                Some(mve) => return mve,
                None => {},
            }
        }

        // spawn threads
        let all_moves = self.get_all_moves(self.on_turn);
        let mut threads: Vec<thread::JoinHandle<(i64, Move)>> = Vec::new();
        for mve in all_moves.iter() {
            let mut new_game = self.clone();
            new_game.do_move(&mve);
            threads.push(
                thread::spawn(move || {
                    let r = new_game.private_get_best_move(depth - 1, depth, CHECK_MATE_SCORE);
                    (r.0 * -1, r.1)
                })
            );
        }

        // receive moves from threads
        let mut best_moves: Vec<(Move, i64)> = Vec::new();
        let mut idx = 0;
        for t in threads {
            let result = t.join().unwrap();
            let mve = all_moves[idx];

            // println!("{} -> {} next expected move: {}", mve.repr(), result.0, result.1.repr());

            let mut insert_at = best_moves.len();
            for (i, item) in best_moves.to_vec().into_iter().enumerate() {
                if result.0 > item.1 {
                    insert_at = i;
                    break;
                }
            }
            best_moves.insert(insert_at, (mve, result.0));

            idx += 1;
        }

        // calculate best move(s) with same highest score
        let mut same_score = 1;
        for mve in best_moves.iter().skip(1) {
            if mve.1 < best_moves[0].1 {
                break;
            }
            same_score += 1;
        }

        let move_idx = rand::thread_rng().gen_range(0..same_score);

        best_moves[move_idx].0
    }

    pub fn private_get_best_move(&self, depth: u8, maximum_depth: u8, score_to_beat: i64) -> (i64, Move) {
        //! Function to calculate best move.
        //! 
        //! Each spawned thread by the function ``get_best_move`` runs this function for it's sub-game.
        //! The Minimax algorithm continues in this function.

        let all_moves = self.get_all_moves(self.on_turn);

        let mut highest_score: i64 = -CHECK_MATE_SCORE;
        let mut best_move = Move::from_long_algebraic_notation(String::from("a1a2"));
        for mve in all_moves.iter() {
            // generate new game from move
            let mut new_game = self.clone();
            new_game.do_move(&mve);

            // calculate the score of the game
            let mut game_score: i64;
            game_score = new_game.get_board_score(new_game.on_turn) * -1;
            if game_score == -CHECK_MATE_SCORE || game_score == CHECK_MATE_SCORE {
                return (game_score, *mve);
            }
            if depth > 1 {
                let r = new_game.private_get_best_move(depth - 1, maximum_depth, (*&highest_score) * -1);
                game_score = r.0 * -1;
            }

            // check if this is the best performing one
            if game_score > CHECK_MATE_SCORE - maximum_depth as i64 {
                game_score -= 1;
            } else if game_score < -CHECK_MATE_SCORE + maximum_depth as i64 {
                game_score += 1;
            }

            // ab-pruning
            if game_score > score_to_beat {
                return (game_score, *mve);
            }

            // update highest score
            if game_score > highest_score {
                highest_score = game_score;
                best_move = *mve;
            }
        }

        (highest_score, best_move)
    }
}