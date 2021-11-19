

#[derive(Debug, PartialEq)]
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

pub struct Move {
    pub from: [i8; 2],
    pub to: [i8; 2],
}
