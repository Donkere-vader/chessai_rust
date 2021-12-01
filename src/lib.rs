mod consts;
mod game;
mod piece;
mod piece_scores;

use pyo3::prelude::*;
use std::collections::{ HashMap };
use game::{ Game };
use consts::{ SearchDepth };


#[pyfunction]
fn get_best_move(fen_code: String) -> HashMap<String, [i8; 2]> {
    let game = Game::from_fen(fen_code);
    let best_move = game.get_best_move(SearchDepth::Shallow);
    let mut map: HashMap<String, [i8; 2]> = HashMap::new();
    map.insert(String::from("from"), best_move.from);
    map.insert(String::from("to"), best_move.to);

    map
}


#[pymodule]
fn chess_ai(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_best_move, m)?)?;

    Ok(())
}
