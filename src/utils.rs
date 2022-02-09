//! Some useful functions for use in other places of the code.

use crate::move_struct::{ Move };
use crate::consts::{ Color };
use crate::piece::{ Piece };
use crate::types::{ Cord };


pub fn walk_offsets(color: &Color, from: Cord, board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, max_distance: Option<u32>, take: bool) -> Vec<Move> {
    //! Walk from the specified from position over the specified board. With the specified offsets to a maximum distance or to encountering 
    //! a piece of the other color.
    //! 
    //! Returns a list of possible moves including take moves if take is true.

    let mut new_moves: Vec<Move> = Vec::new();

    let mut distance;
    for offset in offsets {
        distance = 0;
        let mut current_cord_i8 = [from[0] as i8, from[1] as i8];
        loop {
            current_cord_i8[0] += offset[0];
            current_cord_i8[1] += offset[1];

            // Check if current_coord exists
            if !(0 <= current_cord_i8[0] && current_cord_i8[0] <= 7 && 0 <= current_cord_i8[1] && current_cord_i8[1] <= 7) {
                break;
            }

            let current_cord = [current_cord_i8[0] as usize, current_cord_i8[1] as usize];

            // Check if tile is empty or takable
            match &board[current_cord[1]][current_cord[0]] {
                Some(p) => {
                    if p.color != *color && take {
                        new_moves.push( Move::simple_new(from, current_cord) );
                    }
                    break;
                },
                None => new_moves.push( Move::simple_new(from, current_cord) ),
            }

            // Check if max_distance is not yet reached
            distance += 1;
            if let Some(mx_d) = max_distance {
                if distance == mx_d {
                    break;
                }
            }
        }
    }

    new_moves
}

pub fn with_offsets(color: &Color, from: Cord, board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, has_to_take: bool) -> Vec<Move> {
    //! Check positions to go to with a  certain offset. On the specified board from the specified position.
    //! 
    //! Doesn't add non-take moves to list of possible moves if ``has_to_take`` is true.

    let mut new_moves: Vec<Move> = Vec::new();

    for offset in offsets {
        let new_cord_i8 = [from[0] as i8 + offset[0], from[1] as i8 + offset[1]];

        // Check if new_coord exists
        if !(0 <= new_cord_i8[0] && new_cord_i8[0] <= 7 && 0 <= new_cord_i8[1] && new_cord_i8[1] <= 7) {
            continue;
        }

        let new_cord = [new_cord_i8[0] as usize, new_cord_i8[1] as usize];

        // Check if tile is empty or takable
        match &board[new_cord[1]][new_cord[0]] {
            Some(p) => if p.color != *color { new_moves.push( Move::simple_new(from, new_cord) ); },
            None => if !has_to_take { new_moves.push( Move::simple_new(from, new_cord) ) },
        }
    }

    new_moves
}

pub fn string_square_to_square(string_square: String) -> Cord {
    //! Convert a coordinate to a string square
    //! 
    //! [0, 0] -> a1  
    //! [4, 3] -> e4

    let mut square = [0; 2];
    let string_square_chars = string_square.chars().collect::<Vec<char>>();
    square[0] = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|x| string_square_chars[0] == *x).unwrap();
    square[1] = string_square_chars[1].to_digit(10).unwrap() as usize - 1;

    square
}
