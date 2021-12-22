use crate::piece::{ Piece };

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum MoveType {
    Standard,
    Promote,
    Castle,
    EnPassant
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: [i8; 2],
    pub to: [i8; 2],
    pub move_type: MoveType,
    pub piece: Option<Piece>,
}

impl Move {
    pub fn simple_new(from: [i8; 2], to: [i8; 2]) -> Move {
        Move {
            from: from,
            to: to,
            move_type: MoveType::Standard,
            piece: None,
        }
    }

    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        let from = format!("{}{}", (self.from[0] + 97) as u8 as char, self.from[1] + 1);
        let to = format!("{}{}", (self.to[0] + 97) as u8 as char, self.to[1] + 1);
        format!("<Move ({} -> {})>", from, to)
    }
}
