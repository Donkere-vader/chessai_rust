//! Functions to do moves of type Move.
//! 
//! All functions are in an impl for Game.

use crate::piece::{ Piece };
use crate::consts::{ Color, PieceType, MoveType };
use crate::move_struct::{ Move };
use crate::game::{ Game };


impl Game {
    //! Move implementations

    pub fn apply_moves(&mut self, moves: &Vec<Move>) {
        //! When supplied with a vector of Moves, this function will apply all moves in order.

        for mve in moves.iter() {
            self.do_move(mve);
        }
    }

    fn disable_castle(&mut self, color: &Color) {
        //! Will disable the castling ability (King's and queen's side) for the specified color.

        let mut idxs_to_remove = Vec::new();
        for (idx, piece) in self.castle.iter().enumerate() {
            if piece.color == *color {
                idxs_to_remove.push(idx);
            }
        }

        for (idx, delete_idx) in idxs_to_remove.iter().enumerate() {
            self.castle.remove(delete_idx - idx);
        }
    }

    pub fn do_move(&mut self, mve: &Move) {
        //! Will apply the specified Move to the game.

        let mut score_delta: i64 = 0;
        self.en_passant_target_square = None;
        let piece = self.board[mve.from[1] as usize ][mve.from[0] as usize].unwrap();
        let mut take_piece = None;
        let mut take_piece_cord = [0usize; 2];
        score_delta -= piece.score(mve.from[0] as usize, mve.from[1] as usize, &self.game_phase);
        self.board[mve.from[1] as usize ][mve.from[0] as usize] = None;

        let (mve_type, mve_piece) = mve.get_move_type(Some(&self.castle), self.en_passant_target_square, Some(piece.piece_type));
        match mve_type {
            MoveType::Standard => {
                // update score
                score_delta += piece.score(mve.to[0] as usize, mve.to[1] as usize, &self.game_phase);

                // do move
                // check for disable castle
                let mut pieces_to_check = vec![piece];
                match self.board[mve.to[1] as usize ][mve.to[0] as usize] {
                    Some(p) => {
                        pieces_to_check.push(p);
                        take_piece = Some(p);
                        take_piece_cord = [mve.to[0] as usize, mve.to[1] as usize];
                    },
                    None => {},
                };
                for p in pieces_to_check {
                    if p.piece_type == PieceType::King {
                        self.disable_castle(&p.color);
                    } else {
                        if p.piece_type == PieceType::Rook {
                            let to_remove_piece = Piece { piece_type: if mve.from[0] == 0 { PieceType::Queen } else { PieceType::King }, color: p.color};
                            if self.castle.contains(&to_remove_piece) {
                                self.castle.remove(self.castle.iter().position(|x| *x == to_remove_piece).unwrap());
                            }
                        }
                    }
                }

                self.board[mve.to[1] as usize ][mve.to[0] as usize] = Some(piece);

                if piece.piece_type == PieceType::Pawn {
                    let delta = mve.from[1] as i8 - mve.to[1] as i8;
                    if delta.abs() == 2 {
                        self.en_passant_target_square = Some([mve.from[1] + (delta as usize / 2), mve.to[0]]);
                    }
                }
            },
            MoveType::Promote => {
                // update score
                score_delta += mve_piece.unwrap().score(mve.to[0] as usize, mve.to[1] as usize, &self.game_phase);

                // do move
                self.board[mve.to[1] as usize ][mve.to[0] as usize] = mve_piece;
            },
            MoveType::Castle => {
                // do move
                let mve_piece = mve_piece.unwrap();
                let y = mve.to[1] as usize;
                if mve_piece.piece_type == PieceType::King {
                    score_delta += piece.score(6, y, &self.game_phase);
                    self.board[y][6] = Some(Piece { piece_type: PieceType::King, color: mve_piece.color});
                    self.board[y][5] = Some(Piece { piece_type: PieceType::Rook, color: mve_piece.color});
                    match self.board[y][7] {
                        Some(p) => {
                            score_delta -= p.score(7, y, &self.game_phase);
                            score_delta += p.score(5, y, &self.game_phase);
                        },
                        None => {},
                    }
                    self.board[y][7] = None;
                } else if mve_piece.piece_type == PieceType::Queen {
                    score_delta += piece.score(2, y, &self.game_phase);
                    self.board[y][2] = Some(Piece { piece_type: PieceType::King, color: mve_piece.color});
                    self.board[y][3] = Some(Piece { piece_type: PieceType::Rook, color: mve_piece.color});
                    match self.board[y][0] {
                        Some(p) => {
                            score_delta -= p.score(0, y, &self.game_phase);
                            score_delta += p.score(3, y, &self.game_phase);
                        },
                        None => {},
                    }
                    self.board[y][0] = None;
                }

                self.disable_castle(&mve_piece.color);
            },
            MoveType::EnPassant => {
                // update score
                match self.board[mve.from[1] as usize][mve.to[0] as usize] {
                    Some(p) => {
                        take_piece = Some(p);
                        take_piece_cord = [mve.to[0] as usize, mve.from[1] as usize];
                    },
                    None => {},
                }
                score_delta += piece.score(mve.to[0] as usize, mve.to[1] as usize, &self.game_phase);

                // do move
                self.board[mve.from[1] as usize][mve.to[0] as usize] = None;
                self.board[mve.to[1] as usize][mve.to[0] as usize] = Some(piece);
            },
        }

        match take_piece {
            Some(p) => {
                if p.piece_type == PieceType::King {
                    self.score_white = if p.color == Color::White { -i64::MAX } else { i64::MAX };
                    score_delta = 0;
                } else {
                    score_delta += p.score(take_piece_cord[0], take_piece_cord[1], &self.game_phase);
                }
            },
            None => {},
        };

        match piece.color {
            Color::White => { self.score_white += score_delta },
            Color::Black => { self.score_white -= score_delta },
        };

        if self.on_turn == Color::Black {
            self.fullmove_counter += 1;
        }
        self.on_turn = if self.on_turn == Color::White { Color::Black } else { Color::White };
        self.moves.push(*mve);
    }
}