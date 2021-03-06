//! Struct for a chess Piece
//! 
//! Has functionality for generating all moves an instance of a Piece can do.

use crate::consts::{ PieceType, Color, GamePhase };
use crate::piece_scores::{ SCORE_KING, SCORE_QUEEN, SCORE_ROOK, SCORE_BISHOP, SCORE_KNIGHT, SCORE_PAWN };
use crate::game::{ Game };
use crate::utils::{ with_offsets, walk_offsets };
use crate::move_struct::{ Move };
use crate::types::{ Cord };


#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}


impl Piece {
    pub fn get_all_moves(&self, pos: Cord, game: &Game) -> Vec<Move> {
        //! Return a list of all the moves this piece can do at it's current place.

        get_all_piece_moves(self.piece_type, self.color, pos, game)
    }

    pub fn from_fen(fen_letter: char) -> Piece {
        //! Load a piece from a FEN character.
        //! 
        //! Q = white queen  
        //! k = black king

        let color;
        if fen_letter.to_lowercase().to_string() == fen_letter.to_string() {
            color = Color::Black;
        } else {
            color = Color::White;
        }

        let piece_type = match fen_letter.to_lowercase().to_string().as_ref() {
            "k" => PieceType::King,
            "q" => PieceType::Queen,
            "p" => PieceType::Pawn,
            "n" => PieceType::Knight,
            "r" => PieceType::Rook,
            "b" => PieceType::Bishop,
            _ => panic!("Invalid char `{}`", fen_letter),
        };

        Piece {
            piece_type,
            color,
        }
    }

    pub fn to_fen(self) -> String {
        //! Dumps the piece to a FEN character
        //! 
        //! Q = white queen  
        //! k = black king

        let piece_letter = match self.piece_type {
            PieceType::King => String::from("k"),
            PieceType::Queen => String::from("q"),
            PieceType::Pawn => String::from("p"),
            PieceType::Knight => String::from("n"),
            PieceType::Rook => String::from("r"),
            PieceType::Bishop => String::from("b"),
        };

        match self.color {
            Color::White => piece_letter.to_uppercase(),
            _ => piece_letter,
        }
    }

    pub fn score(&self, pos: Cord, game_phase: &GamePhase) -> i64 {
        //! Determine the value of the piece's current position.

        let x = pos[0];
        let mut y = pos[1];

        if self.color == Color::Black {
            y = 7 - y;
        }

        match self.piece_type {
            PieceType::King => SCORE_KING[*game_phase as usize][y][x],
            PieceType::Queen => SCORE_QUEEN[*game_phase as usize][y][x] + 900,
            PieceType::Bishop => SCORE_BISHOP[*game_phase as usize][y][x] + 300,
            PieceType::Knight => SCORE_KNIGHT[*game_phase as usize][y][x] + 300,
            PieceType::Rook => SCORE_ROOK[*game_phase as usize][y][x] + 500,
            PieceType::Pawn => SCORE_PAWN[*game_phase as usize][y][x],
        }
    }

    pub fn unicode_piece(&self) -> String {
        //! Return the unicode character for this piece.

        match self.piece_type {
            PieceType::King => String::from("???"),
            PieceType::Queen => String::from("???"),
            PieceType::Bishop => String::from("???"),
            PieceType::Knight => String::from("???"),
            PieceType::Rook => String::from("???"),
            PieceType::Pawn => String::from("???"),
        }
    }

    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        //! Simple way to represent piece in the console.

        format!("<Piece {:?} {:?}>", self.piece_type, self.color)
    }
}


pub fn get_all_piece_moves(piece_type: PieceType, color: Color, pos: Cord, game: &Game) -> Vec<Move> {
    //! Get a list of all the moves a piece of this type and color can do from the specified position in the specified game.

    let mut moves: Vec<Move> = Vec::new();
    
    match piece_type {
        PieceType::King => {
            // standard moves
            moves.extend(with_offsets(&color, pos, game.board, vec![[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]], false));

            // castle moves
            if !game.square_is_attacked(pos, if color == Color::White { Color::Black } else { Color::White }) {
                let y = if color == Color::White { 0 } else { 7 };
                let other_color = if color == Color::White { Color::Black } else { Color::White };
                let king = Piece { piece_type: PieceType::King, color};
                let queen = Piece { piece_type: PieceType::Queen, color};
                if game.castle.contains(&king) &&
                    game.board[y][5].is_none() && !game.square_is_attacked([5, y], other_color) && game.board[y][6].is_none() && !game.square_is_attacked([6, pos[1]], other_color) {
                        moves.push(Move { from: pos, to: [7, y], piece: Some(king)});
                }
                if game.castle.contains(&queen) &&
                    game.board[y][1].is_none() && game.board[y][2].is_none() && !game.square_is_attacked([2, pos[1]], other_color) && game.board[y][3].is_none() && !game.square_is_attacked([3, pos[1]], other_color) {
                        moves.push(Move { from: pos, to: [0, y], piece: Some(queen)});
                }
            }
        },
        PieceType::Queen => moves.extend(walk_offsets(&color, pos, game.board, vec![[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]], None, true)),
        PieceType::Bishop => moves.extend(walk_offsets(&color, pos, game.board, vec![[1, 1], [-1, 1], [-1, -1], [1, -1]], None, true)),
        PieceType::Knight => moves.extend(with_offsets(&color, pos, game.board, vec![[-1, 2], [1, 2], [2, 1], [2, -1], [1, -2], [-1, -2], [-2, -1], [-2, 1]], false)),
        PieceType::Rook => moves.extend(walk_offsets(&color, pos, game.board, vec![[1, 0], [0, 1], [-1, 0], [0, -1]], None, true)),
        PieceType::Pawn => {
            // can take pieces?
            moves.extend(with_offsets(&color, pos, game.board, if color == Color::White { vec![[1, 1], [-1, 1]] } else { vec![[1, -1], [-1, -1]] }, true));

            if (color == Color::White && pos[1] == 6 && game.board[7][pos[0]].is_none() ) || (color == Color::Black && pos[1] == 1 && game.board[0][pos[0]].is_none()) {
                // promote
                let to_y = if color == Color::White { 7 } else { 0 };
                for piece_type in [PieceType::Queen, PieceType::Knight, PieceType::Rook, PieceType::Bishop] {
                    moves.push(Move { from: pos, to: [pos[0], to_y], piece: Some(Piece { piece_type, color }) } );
                }
            } else {
                // standard moves
                moves.extend(walk_offsets(
                    &color,
                    pos,
                    game.board,
                    if color == Color::White { vec![[0, 1]] } else { vec![[0, -1]] },
                    if (color == Color::White && pos[1] == 1) || (color == Color::Black && pos[1] == 6) { Some(2) } else { Some(1) },
                    false,
                ));
            }

            // en passant
            if game.en_passant_target_square.is_some() {
                for offset in if color == Color::White { vec![[1, 1], [-1, 1]] } else { vec![[1, -1], [-1, -1]] } {
                    let cord = [(pos[0] as i8 + offset[0]) as usize, (pos[1] as i8 + offset[1]) as usize];
                    if game.en_passant_target_square.unwrap() == cord {
                        moves.push(Move::simple_new(pos, cord));
                    }
                }
            }
        },
    }

    moves
}
