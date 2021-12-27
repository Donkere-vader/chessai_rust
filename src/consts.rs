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

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum MoveType {
    Standard,
    Promote,
    Castle,
    EnPassant
}
