use crate::piece::{ Piece };
use crate::consts::{ Color, GamePhase };
use crate::utils::{ string_square_to_square };
use crate::game::{ Game };


impl Game {
    pub fn from_fen(fen_code: String) -> Game {
        let splitted_fen = fen_code.split_whitespace().take(6).collect::<Vec<&str>>();
        let mut board: [[Option<Piece>; 8]; 8] = Default::default();
        let mut castle_vec = Vec::new();
        let on_turn;
        let en_passant_target_square;
        let fullmove_counter;

        if let [board_string, on_turn_fen_let, castling, en_passant_target_square_string, _halfmove_clock, fullmove_counter_string] = &splitted_fen[..] {
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

            for piece_char in castling.chars() {
                if piece_char == '-' { break; }
                castle_vec.push(Piece::from_fen(piece_char));
            }

            if *en_passant_target_square_string != "-" {
                en_passant_target_square = Some(string_square_to_square(en_passant_target_square_string.to_string()));
            } else {
                en_passant_target_square = None;
            }

            fullmove_counter = fullmove_counter_string.parse::<usize>().unwrap();
        } else {
            panic!("Illegal FEN code");
        }

        let mut new_game = Game {
            board: board,
            on_turn: on_turn,
            castle: castle_vec,
            en_passant_target_square: en_passant_target_square,
            score_white: 0,
            moves: Vec::new(),
            fullmove_counter: fullmove_counter,
            game_phase: GamePhase::StartGame,
        };

        new_game.calculate_board_score();
        new_game.calculate_game_phase();

        new_game
    }

    #[allow(dead_code)]
    pub fn to_fen(&self) -> String {
        let mut board_string = String::new();

        let mut empty_spaces;
        let mut rank_idx = 0;
        for rank in self.board.iter().rev() {
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

        let mut castling_string = String::new();
        if self.castle.len() > 0 {
            for piece in self.castle.iter() {
                castling_string += &piece.to_fen();
            }
        } else {
            castling_string = String::from("-");
        }

        format!("{} {} {} - 0 1", board_string, on_turn, castling_string)
    }
}
