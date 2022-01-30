#![warn(missing_docs)]

//! Chess AI written in Rust.  
//! This AI was written for my Profielwerkstuk (dutch highschool thesis)

use uci::{ uci };
use argparse::{ArgumentParser, Store};

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
    let mut mode = String::new();

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        ap.set_description("Run chess AI.");

        ap.refer(&mut mode)
            .add_option(&["--mode"], Store,
            "Mode to run chess AI in. Choose from: [UCI]");
        ap.parse_args_or_exit();

    }

    if mode == "uci".to_string() {
        uci();
    } else {
        panic!("'{}' is not a valid mode.", mode);
    }
}
