use colored::*;
use crate::piece::{ Piece };
use crate::consts::{ Color, Move, PieceType };


pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub on_turn: Color,
}


impl Game {
    pub fn from_fen(fen_code: String) -> Game {
        let splitted_fen = fen_code.split_whitespace().take(6).collect::<Vec<&str>>();
        let mut board: [[Option<Piece>; 8]; 8] = Default::default();
        let on_turn;

        if let [board_string, on_turn_fen_let, _castling, _en_passant_target_square, _haflmove_clock, _fullmove_counter] = &splitted_fen[..] {
            let mut y = 0;
            for rank in board_string.split("/") {
                let mut x = 0;
                for chr in rank.chars() {
                    match chr.to_digit(10) {
                        None => {
                            board[y][x] = Some(Piece::from_fen(chr));
                            x += 1;
                        },
                        Some(num) => {
                            for x_delta in 0..num {
                                board[y][x + x_delta as usize] = None;
                            }
                            x += num as usize;
                        },
                    }
                }
                y += 1;
            }
            
            on_turn = if **on_turn_fen_let == String::from("w") { Color::White } else { Color::Black };
        } else {
            panic!("Illegal FEN code");
        }

        Game {
            board: board,
            on_turn: on_turn,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut board_string = String::new();

        let mut empty_spaces;
        let mut rank_idx = 0;
        for rank in &self.board {
            empty_spaces = 0;
            for piece in rank {
                match piece {
                    Some(p) => {
                        if empty_spaces > 0 {
                            board_string += &empty_spaces.to_string();
                            empty_spaces = 0;
                        }
                        board_string += &p.to_fen();
                    },
                    None => empty_spaces += 1,
                }
            }
            if empty_spaces > 0 {
                board_string += &empty_spaces.to_string();
            }
            if rank_idx != 7 {
                board_string += "/";
            }
            rank_idx += 1;
        }

        let on_turn = match self.on_turn {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
        };

        format!("{} {} KQkq - 0 1", board_string, on_turn)
    }
    
    pub fn show_board(&self) {
        let mut y = 0;
        for rank in &self.board {
            let mut x = 0;
            for piece in rank {
                let mut tile_color = Color::White;
                let tile = match piece {
                    Some(p) => {
                        tile_color = p.color;
                        format!(" {} ", &p.unicode_piece())
                    },
                    _ => String::from("   "),
                };

                if (y + x) % 2 == 0 {
                    print!(
                        "{}",
                        match tile_color {
                            Color::White => tile.on_truecolor(181, 136, 99).blue(),
                            Color::Black => tile.on_truecolor(181, 136, 99).red(),
                        }
                    );
                } else {
                    print!(
                        "{}",
                        match tile_color {
                            Color::White => tile.on_truecolor(240, 217, 181).blue(),
                            Color::Black => tile.on_truecolor(240, 217, 181).red(),
                        }
                    );
                }
                x += 1;
            }
            y += 1;
            println!();
        }
    }

    pub fn get_board_score(&self, color: Color) -> f64 {
        let mut board_score: f64 = 0.0;

        let mut white_king_present = false;
        let mut black_king_present = false;

        let mut y = 0;
        let mut x;
        for rank in &self.board {
            x = 0;
            for piece in rank {
                match piece {
                    Some(p) => {
                        let piece_score = p.score(x, y);
                        if p.color == color {
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
            board_score = f64::INFINITY;
            if (!white_king_present && color == Color::White) || (!black_king_present && color == Color::Black) {
                board_score *= -1.0;
            }
        }

        board_score
    }

    fn get_all_moves(&self) -> Vec<Move> {
        vec![]
    }

    pub fn get_best_move(&self, current_depth: u8, initial_depth: u8) -> Move {

        Move { from: [1, 1], to: [1, 1] }
    }
}
