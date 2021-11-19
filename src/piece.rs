use crate::consts::{ PieceType, Color, Move };
use crate::piece_scores::{ SCORE_KING, SCORE_QUEEN, SCORE_ROOK, SCORE_BISHOP, SCORE_KNIGHT, SCORE_PAWN };


pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}


impl Piece {
    // pub fn repr(&self) -> String {
    //     format!("<Piece {:?} {:?}>", self.piece_type, self.color)
    // }

    pub fn get_all_moves(&self, board: [[Option<Piece>; 8]; 8]) -> Vec<Move> {
        fn walk_offset(offset: [i8; 2]) -> Vec<[i8; 2]> {
            vec![]
        }

        vec![]
    }

    pub fn from_fen(fen_letter: char) -> Piece {
        let color;
        if fen_letter.to_lowercase().to_string() == fen_letter.to_string() {
            color = Color::Black;
        } else {
            color = Color::White;
        }

        let piece_type = match fen_letter.to_lowercase().to_string().as_ref() {
            "k" => PieceType::King,
            "q" => PieceType::Queen,
            "p" => PieceType::Pawn,
            "n" => PieceType::Knight,
            "r" => PieceType::Rook,
            _ => PieceType::Bishop,
        };

        Piece {
            piece_type: piece_type,
            color: color,
        }
    }

    pub fn to_fen(&self) -> String {
        let piece_letter = match self.piece_type {
            PieceType::King => String::from("k"),
            PieceType::Queen => String::from("q"),
            PieceType::Pawn => String::from("p"),
            PieceType::Knight => String::from("n"),
            PieceType::Rook => String::from("r"),
            PieceType::Bishop => String::from("b"),
        };

        match self.color {
            Color::White => piece_letter.to_uppercase(),
            _ => piece_letter,
        }
    }

    pub fn score(&self, x: usize, mut y: usize) -> f64 {
        y = match self.color {
            Color::White => y,
            Color::Black => 7 - y,
        };

        match self.piece_type {
            PieceType::King => SCORE_KING[y][x],
            PieceType::Queen => SCORE_QUEEN[y][x] * 9.0,
            PieceType::Bishop => SCORE_BISHOP[y][x] * 3.0,
            PieceType::Knight => SCORE_KNIGHT[y][x] * 3.0,
            PieceType::Rook => SCORE_ROOK[y][x] * 5.0,
            PieceType::Pawn => SCORE_PAWN[y][x],
        }
    }

    pub fn unicode_piece(&self) -> String {
        match self.piece_type {
            PieceType::King => String::from("♚"),
            PieceType::Queen => String::from("♛"),
            PieceType::Bishop => String::from("♝"),
            PieceType::Knight => String::from("♞"),
            PieceType::Rook => String::from("♜"),
            PieceType::Pawn => String::from("♟︎"),
        }
    }
}
