mod consts;
mod game;
mod piece;
mod piece_scores;
mod utils;

use pyo3::prelude::*;
use std::collections::{ HashMap };
use game::{ Game };
use consts::{ Color };

#[pyfunction]
fn get_best_move(fen_code: String, depth: u8, verbose: bool) -> (String, HashMap<String, [i8; 2]>) {
    let mut game = Game::from_fen(fen_code);
    let best_move = game.get_best_move(depth, verbose);
    let mut map: HashMap<String, [i8; 2]> = HashMap::new();
    map.insert(String::from("from"), best_move.from);
    map.insert(String::from("to"), best_move.to);

    for y in 0..8i8 {
        for x in 0..8i8 {
            if game.square_is_attacked([x, y], Color::White) {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
    }

    game.do_move(&best_move);

    (game.to_fen(), map)
}


#[pymodule]
fn chess_ai(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_best_move, m)?)?;

    Ok(())
}
