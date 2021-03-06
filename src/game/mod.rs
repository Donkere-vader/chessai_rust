//! Game module for keeping track of the game state and calculating the best moves

pub mod storage;
pub mod misc;
pub mod evaluation;
pub mod moving;
pub mod best_move;

use crate::piece::{ Piece };
use crate::consts::{ Color, GamePhase };
use crate::move_struct::{ Move };
use crate::types::{ Cord };


#[derive(Clone)]
pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub on_turn: Color,
    pub castle: Vec<Piece>,
    pub en_passant_target_square: Option<Cord>,
    pub moves: Vec<Move>,
    pub score_white: i64,
    pub fullmove_counter: usize,
    pub game_phase: GamePhase,
}
