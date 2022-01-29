//! Struct make handling moves easier.
//!
//! Easy loading, dumping and working with the move.

use crate::consts::{ MoveType, PieceType, Color };
use crate::types::{ Cord };
use crate::piece::{ Piece };
use std::fmt;


#[derive(Copy, Clone)]
pub struct Move {
    pub from: Cord,
    pub to: Cord,
    pub piece: Option<Piece>,
}

impl Move {
    pub fn simple_new(from: Cord, to: Cord) -> Move {
        //! Create a basic new Move (a to b nothing special)
        Move {
            from: from,
            to: to,
            piece: None,
        }
    }

    pub fn from_long_algebraic_notation(notation: String) -> Move {
        //! Load a move from the long algebraic notation
        //! 
        //! Examples of long algebraic notation:
        //! a2a4 (white pawn from a2 to a4)
        //! a7a8q (white pawn from a7 to a8 promoting to queen)

        let notation = notation.chars().collect::<Vec<char>>();
        let from_str = (notation[0], notation[1].to_digit(10).unwrap() - 1);
        let to_str = (notation[2], notation[3].to_digit(10).unwrap() - 1);

        let from: Cord = [
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|&x| x == from_str.0).unwrap(),
            from_str.1  as usize,
        ];

        let to: Cord  = [
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|&x| x == to_str.0).unwrap(),
            to_str.1 as usize,
        ];


        let mut piece = None;
        if notation.len() > 4 {
            piece = Some(Piece::from_fen(notation[4]));
        }


        Move {
            from: from,
            to: to,
            piece: piece,
        }
    }

    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        //! Simple ray to represent move in console.
        //! Used for debugging.
        format!("<Move {}>", self.long_algebraic_notation())
    }

    pub fn get_move_type(&self, castling: Option<&Vec<Piece>>, en_passant_target_square: Option<Cord>, piece_type: Option<PieceType>) -> (MoveType, Option<Piece>) {
        //! Determine what type of move this move is.
        //! 
        //! Is it a castle move? A promotion? etc.
        
        if self.piece.is_some() {
            if self.from[1] == 6 || self.from[1] == 1 {
                return (MoveType::Promote, self.piece);
            }
        };

        if castling.is_some() && self.from[0] == 4 && (self.from[1] == 0 || self.from[1] == 7) {
            let castling = castling.unwrap();
            let color = if self.from[1] == 0 { Color::White } else { Color::Black };
            let king = Piece { piece_type: PieceType::King, color: color};
            let queen = Piece { piece_type: PieceType::Queen, color: color};
            if (self.to[0] == 6 || self.to[0] == 7) && castling.contains(&king) {
                return (MoveType::Castle, Some(king));
            } else if (self.to[0] == 2 || self.to[0] == 0) && castling.contains(&Piece { piece_type: PieceType::Queen, color: color}) {
                return (MoveType::Castle, Some(queen));
            }
        }

        if en_passant_target_square.is_some() && piece_type.is_some() && piece_type.unwrap() == PieceType::Pawn && en_passant_target_square.unwrap() == self.to {
            return (MoveType::EnPassant, None);
        }

        (MoveType::Standard, None)
    }

    pub fn long_algebraic_notation(&self) -> String {
        //! Dump the move to a long algebraic notation.
        //! 
        //! Examples of long algebraic notation:
        //! a2a4 (white pawn from a2 to a4)
        //! a7a8q (white pawn from a7 to a8 promoting to queen)

        let from = format!("{}{}", (self.from[0] + 97) as u8 as char, self.from[1] + 1);
        let to = format!("{}{}", (self.to[0] + 97) as u8 as char, self.to[1] + 1);

        let promotion = match self.get_move_type(None, None, None).0 {
            MoveType::Promote => { 
                match self.piece {
                    Some(p) => {
                        match p.piece_type {
                            PieceType::King => "k",
                            PieceType::Queen => "q",
                            PieceType::Pawn => "p",
                            PieceType::Knight => "n",
                            PieceType::Rook => "r",
                            PieceType::Bishop => "b",
                        }
                    },
                    None => "",
                }
            },
            _ => "",
        };

        format!("{}{}{}", from, to, promotion)
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.repr())
    }
}
