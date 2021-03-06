//! Collection of constants and enums that are used project-wide.

pub const SEARCH_DEPTH: u8 = 5;
pub const CHECK_MATE_SCORE: i64 = i64::MAX;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveType {
    Standard,
    Promote,
    Castle,
    EnPassant,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum GamePhase {
    Start = 0,
    Mid = 1,
    End = 2,
}
