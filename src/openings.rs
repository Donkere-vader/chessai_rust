//! Struct for openings database
//! 
//! This struct loads all the know openings from the openings file.
//! And provides search functionality.

use crate::move_struct::{ Move };
use std::fs;
use std::path::{ Path };
use rand::Rng;

#[derive(Clone)]
struct Opening {
    // eco: String,
    // name: String,
    moves: Vec<Move>,
}

#[derive(Clone)]
pub struct 

OpeningsDatabase {
    openings: Vec<Opening>,
}


impl OpeningsDatabase {
    pub fn new() -> OpeningsDatabase {
        //! Open new OpeningsDatabase
        //! 
        //! Automatically opens the database file. Or panics it if it doesn't exist.

        let path = Path::new("./chess_openings.txt");
        if !path.exists() { panic!("No openings database found!"); }

        let contents = fs::read_to_string(path).expect("Failed to read file");
        let mut openings = Vec::new();
        let mut eco = None;
        let mut name = None;
        let mut moves = None;
        for line in contents.split('\n') {
            let splitted_line = line.split(' ').collect::<Vec<&str>>();
            let line_type = splitted_line[0];
            if line_type == "ECO" {
                eco = Some(splitted_line[1]);
            } else if line_type == "NAME" {
                let mut name_string = String::new();
                for item in splitted_line.iter().skip(1) {
                    name_string += item;
                }
                name = Some(name_string)
            } else if line_type == "UCI" {
                let mut new_moves = Vec::new();
                for (idx, item) in splitted_line.iter().enumerate() {
                    if idx == 0 { continue }
                    new_moves.push(Move::from_long_algebraic_notation(item.to_string()));
                }
                moves = Some(new_moves)
            }

            if eco.is_some() && name.is_some() && moves.is_some() {
                openings.push(Opening {
                    // eco: eco.unwrap().to_string(),
                    // name: name.unwrap().to_string(),
                    moves: moves.unwrap(),
                });
                eco = None;
                name = None;
                moves = None;
            }
        }

        OpeningsDatabase {
            openings
        }
    }

    pub fn find_opening(&self, moves: &[Move]) -> Option<Move> {
        //! Search for a opening with the specified move history.
        //! 
        //! Returns Some(Move) if it found anything, else None.

        let mut matching_openings = Vec::new();
        for opening in self.openings.iter() {
            if opening.moves.len() <= moves.len() { continue };
            let mut identical = true;
            for (idx, mve) in moves.iter().enumerate() {
                if opening.moves[idx].from != mve.from || opening.moves[idx].to != mve.to {
                    identical = false;
                    break;
                }
            }

            if identical {
                matching_openings.push(opening);
            }
        }

        if !matching_openings.is_empty() {
            let open_idx = rand::thread_rng().gen_range(0..matching_openings.len());
            return Some(matching_openings[open_idx].moves[moves.len()]);
        }

        None
    }
}
