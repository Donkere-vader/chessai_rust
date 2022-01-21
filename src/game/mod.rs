pub mod storage;
pub mod misc;
pub mod evaluation;
pub mod moving;
pub mod best_move;

use crate::piece::{ Piece };
use crate::consts::{ Color, GameFase };
use crate::move_struct::{ Move };

#[derive(Clone)]
pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub on_turn: Color,
    pub castle: Vec<Piece>,
    pub en_passant_target_square: Option<[i8; 2]>,
    pub moves: Vec<Move>,
    pub score_white: i64,
    pub fullmove_counter: usize,
    pub game_fase: GameFase,
}
