//! Functions that are used to determine if the current game state is any good for the AI.
//! 
//! All functions are in an impl for Game.

use crate::consts::{ GamePhase };
use crate::game::{ Game };
use crate::consts::{ Color, CHECK_MATE_SCORE };
use crate::move_struct::{ Move };
use crate::consts::{ PieceType };


impl Game {
    //! Evaluation implementations

    pub fn calculate_game_phase(&mut self) {
        //! Calculates the current game phase.

        let mut n_pieces_midfield = 0;
        let mut n_pieces = 0;
        for y in 0..8 {
            for x in 0..8 {
                match self.board[y][x] {
                    Some(_) => {
                        n_pieces += 1;
                        if y >= 2 && y <= 5 {
                            n_pieces_midfield += 1;
                        }
                    },
                    None => {},
                }
            }
        }

        if n_pieces_midfield > 8 || self.fullmove_counter > 15 {
            self.game_phase = GamePhase::MidGame;
        }

        if n_pieces < 12 {
            self.game_phase = GamePhase::EndGame;
        }
    }

    pub fn get_board_score(&mut self, color: Color) -> i64 {
        //! Returns the board score for the specified color

        match color {
            Color::White => self.score_white,
            Color::Black => self.score_white * -1,
        }
    }

    pub fn calculate_board_score(&mut self) {
        //! Calculates the score for the current board.
        //! 
        //! Only used when loading the board.
        //! Afterwards the score is simply updated by only checking the new values for the moved pieces.
        let mut board_score: i64 = 0;

        let mut white_king_present = false;
        let mut black_king_present = false;

        let mut y = 0;
        let mut x;
        for rank in self.board.iter() {
            x = 0;
            for piece in rank.iter() {
                match piece {
                    Some(p) => {
                        let piece_score = p.score(x, y, &self.game_phase);
                        if p.color == Color::White {
                            board_score += piece_score;
                        } else {
                            board_score -= piece_score;
                        }

                        if p.piece_type == PieceType::King {
                            match p.color {
                                Color::White => white_king_present = true,
                                Color::Black => black_king_present = true,
                            }
                        }
                    },
                    None => {},
                }
                x += 1;
            }
            y += 1;
        }

        if !white_king_present || !black_king_present {
            board_score = CHECK_MATE_SCORE;
            if !white_king_present {
                board_score *= -1;
            }
        }

        self.score_white = board_score;
    }

    pub fn get_all_moves(&self, color: Color) -> Vec<Move> {
        //! Returns a vector of Moves that the specified color can make at this point in the game.

        let mut all_moves = Vec::new();

        for (y, rank) in (&self.board).iter().enumerate() {
            for (x, piece) in rank.iter().enumerate() {
                match piece {
                    Some(p) => if color == p.color {
                        // sort so that pawns will get checked last
                        match p.piece_type {
                            PieceType::Pawn => { all_moves.extend(p.get_all_moves(x as i8, y as i8, &self)) },
                            _ => {
                                for mve in p.get_all_moves(x as i8, y as i8, &self) {
                                    all_moves.insert(0, mve);
                                }
                            }
                        }
                    },
                    None => {},
                }
            }
        }

        all_moves
    }
}