#![warn(missing_docs)]

//! Chess AI written in Rust.  
//! This AI was written for my Profielwerkstuk (dutch highschool thesis)

use uci::{ uci };

mod consts;
mod game;
mod piece;
mod piece_scores;
mod move_struct;
mod logger;
mod utils;
mod openings;
mod types;
mod uci;
#[cfg(test)]
mod tests;

fn main() {
    uci();
}
