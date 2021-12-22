use colored::*;
use crate::piece::{ Piece };
use crate::consts::{ Color, Move, PieceType };
use std::thread;

const CHECK_MATE_SCORE: f64 = 10000.0;

#[derive(Copy, Clone)]
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
            for rank in board_string.rsplit("/") {
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

    #[allow(dead_code)]
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
    
    #[allow(dead_code)]
    pub fn show_board(&self, highlight: Option<Vec<[i8; 2]>>, seen_from: Color) {
        let highlight = match highlight {
            Some(h) => h,
            None => Vec::new(),
        };

        for mut y in 0..8 {
            if seen_from == Color::White { y = 7 - y }
            for mut x in 0..8 {
                if seen_from == Color::Black { x = 7 - x }
                let piece = &self.board[y][x];
                let mut tile_color = Color::White;
                let tile = match piece {
                    Some(p) => {
                        tile_color = p.color;
                        format!(" {} ", &p.unicode_piece())
                    },
                    _ => String::from("   "),
                };

                // println!("{} {} {:?}", x, y, highlight.contains(&[x as i8, y as i8]));

                if highlight.contains(&[x as i8, y as i8]) {
                    print!(
                        "{}",
                        match tile_color {
                            Color::White => tile.on_green().blue(),
                            Color::Black => tile.on_green().red(),
                        }
                    );
                } else if (y + x) % 2 == 0 {
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
            }
            println!(" {}", y + 1);
        }
        match seen_from {
            Color::White => println!(" a  b  c  d  e  f  g  h"),
            Color::Black => println!(" h  g  f  e  d  c  b  a"),
        }
    }

    pub fn do_move(&mut self, mve: &Move) {
        let mut done = false;
        let piece = self.board[mve.from[1] as usize ][mve.from[0] as usize];
        match piece {
            Some(p) => {
                if p.piece_type == PieceType::Pawn && (mve.to[1] == 0 || mve.to[1] == 7) {
                    self.board[mve.to[1] as usize ][mve.to[0] as usize] = Some(Piece { piece_type: PieceType::Queen, color: p.color });
                    self.board[mve.from[1] as usize ][mve.from[0] as usize] = None;
                    done = true;
                }
            },
            None => {},
        }

        if !done {
            self.board[mve.to[1] as usize ][mve.to[0] as usize] = piece;
            self.board[mve.from[1] as usize ][mve.from[0] as usize] = None;    
        }

        self.on_turn = if self.on_turn == Color::White { Color::Black } else { Color::White };
    }

    pub fn get_board_score(&self, color: Color) -> f64 {
        let mut board_score: f64 = 0.0;

        let mut white_king_present = false;
        let mut black_king_present = false;

        let mut y = 0;
        let mut x;
        for rank in self.board.iter() {
            x = 0;
            for piece in rank.iter() {
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
            board_score = CHECK_MATE_SCORE;
            if (!white_king_present && color == Color::White) || (!black_king_present && color == Color::Black) {
                board_score *= -1.0;
            }
        }

        board_score
    }

    pub fn get_all_moves(&self, color: Color) -> Vec<Move> {
        let mut all_moves = Vec::new();

        for (y, rank) in (&self.board).iter().enumerate() {
            for (x, piece) in rank.iter().enumerate() {
                match piece {
                    Some(p) => if color == p.color {
                        // sort so that pawns will get checked last
                        match p.piece_type {
                            PieceType::Pawn => { all_moves.extend(p.get_all_moves(x as i8, y as i8, &self.board)) },
                            _ => {
                                for mve in p.get_all_moves(x as i8, y as i8, &self.board) {
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

    pub fn get_number_of_pieces(&self) -> u8 {
        let mut total = 0;

        for y in 0..8 {
            for x in 0..8 {
                match self.board[y][x] {
                    Some(_) => total += 1,
                    None => {},
                }
            }
        }

        total
    }

    pub fn get_best_move(&self, depth: u8, verbose: bool) -> Move {
        let all_moves = self.get_all_moves(self.on_turn);

        let mut threads: Vec<thread::JoinHandle<f64>> = Vec::new();

        let n_pieces = self.get_number_of_pieces();
        if verbose { println!("N Pieces: {}\nSearch depth: {}", n_pieces, depth); }

        for mve in all_moves.iter() {
            let mut new_game = *self;
            new_game.do_move(&mve);
            threads.push(
                thread::spawn(move || {
                    let game_score = new_game.private_get_best_move(depth - 1, depth, CHECK_MATE_SCORE) * -1.0;
                    game_score
                })
            );
        }

        let mut best_move = all_moves[0];
        let mut highest_score: f64 = -CHECK_MATE_SCORE;

        let mut idx = 0;
        let threads_len = threads.len();
        for t in threads {
            let result = t.join().unwrap();
            if verbose { print!("{: <5} {: <20} -> {: <20}", format!("{}/{}", idx + 1, threads_len), all_moves[idx].repr(), result); }
            if result > highest_score {
                // highest_backtrack = backtrack;
                best_move = all_moves[idx];
                highest_score = result;
                if verbose { print!("Best found yet"); }
            }
            if verbose { println!(); }

            idx += 1;
        }


        if verbose { println!("Best move: {}", best_move.repr()); }
        best_move
    }

    pub fn private_get_best_move(&self, depth: u8, maximum_depth: u8, score_to_beat: f64) -> f64 {
        let all_moves = self.get_all_moves(self.on_turn);

        let mut highest_score: f64 = -CHECK_MATE_SCORE;
        for mve in all_moves.iter() {
            // generate new game from move
            let mut new_game = *self;
            new_game.do_move(&mve);

            // calculate the score of the game
            let mut game_score: f64;
            game_score = new_game.get_board_score(new_game.on_turn) * -1.0;
            if game_score == -CHECK_MATE_SCORE {
                return CHECK_MATE_SCORE * -1.0;
            }
            if depth != 0 {
                game_score = new_game.private_get_best_move(depth - 1, maximum_depth, (*&highest_score) * -1.0) * -1.0;
            }

            // check if this is the best peforming one
            if game_score > CHECK_MATE_SCORE - maximum_depth as f64 {
                game_score -= 1.0;
            } else if game_score < -CHECK_MATE_SCORE + maximum_depth as f64 {
                game_score += 1.0;
            }
            if game_score > highest_score {
                highest_score = game_score;
            }

            // ab-pruning
            if highest_score > score_to_beat {
                return highest_score;
            }
        }

        highest_score
    }
}
