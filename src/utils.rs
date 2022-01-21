use crate::move_struct::{ Move };
use crate::consts::{ Color };
use crate::piece::{ Piece };


pub fn walk_offsets(color: &Color, from: [i8; 2], board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, max_distance: Option<u32>, take: bool) -> Vec<Move> {
    let mut new_moves: Vec<Move> = Vec::new();

    let mut distance;
    for offset in offsets {
        distance = 0;
        let mut current_coord = *&from;
        loop {
            current_coord[0] += offset[0];
            current_coord[1] += offset[1];

            // Check if current_coord exists
            if !(0 <= current_coord[0] && current_coord[0] <= 7) || !(0 <= current_coord[1] && current_coord[1] <= 7) {
                break;
            }

            // Check if tile is empty or takable
            match &board[current_coord[1] as usize][current_coord[0] as usize] {
                Some(p) => {
                    if p.color != *color && take {
                        new_moves.push( Move::simple_new(*&from, *&current_coord) );
                    }
                    break;
                },
                None => new_moves.push( Move::simple_new(*&from, *&current_coord) ),
            }

            // Check if max_distance is not yet reached
            distance += 1;
            match max_distance {
                Some(mx_d) => {
                    if distance == mx_d {
                        break;
                    }
                }
                _ => {},
            }
        }
    }

    new_moves
}

pub fn with_offsets(color: &Color, from: [i8; 2], board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, has_to_take: bool) -> Vec<Move> {
    let mut new_moves: Vec<Move> = Vec::new();

    for offset in offsets {
        let new_coord = [from[0] + offset[0], from[1] + offset[1]];

        // Check if new_coord exists
        if !(0 <= new_coord[0] && new_coord[0] <= 7) || !(0 <= new_coord[1] && new_coord[1] <= 7) {
            continue;
        }

        // Check if tile is empty or takable
        match &board[new_coord[1] as usize][new_coord[0] as usize] {
            Some(p) => if p.color != *color { new_moves.push( Move::simple_new(*&from, *&new_coord) ); },
            None => if !has_to_take { new_moves.push( Move::simple_new(*&from, *&new_coord) ) },
        }
    }

    new_moves
}

pub fn string_square_to_square(string_square: String) -> [i8; 2] {
    let mut square = [0i8; 2];
    let string_square_chars = string_square.chars().collect::<Vec<char>>();
    square[0] = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|x| string_square_chars[0] == *x).unwrap() as i8;
    square[1] = string_square_chars[1].to_digit(10).unwrap() as i8 - 1;

    square
}
