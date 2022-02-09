#![warn(missing_docs)]

//! Chess AI written in Rust.  
//! This AI was written for my Profielwerkstuk (dutch highschool thesis)

use uci::{ uci };
use argparse::{ArgumentParser, Store, StoreOption };
use benchmark::{ run_benchmarks };

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
mod benchmark;
#[cfg(test)]
mod tests;

fn main() {
    let mut mode = String::new();
    let mut save_file = None;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();

        ap.set_description("Run chess AI.");

        ap.refer(&mut mode)
            .add_argument("MODE", Store,
            "Mode to run chess AI in. Choose from: [UCI, benchmark]");
        ap.refer(&mut save_file)
            .add_option(&["-s", "--save"], StoreOption,
            "File to save output to");
        ap.parse_args_or_exit();

    }

    if mode.to_uppercase() == *"UCI".to_string() {
        uci();
    } else if vec!["bench".to_string(), "benchmark".to_string()].contains(&mode.to_lowercase()) {
        run_benchmarks(save_file);
    } else {
        panic!("'{}' is not a valid mode.", mode);
    }
}
