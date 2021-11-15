#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}

impl Piece {
    pub fn repr(self) -> String {
        format!("<Piece {:?} {:?}>", self.piece_type, self.color)
    }

    pub fn from_fen(fen_letter: String) -> Piece {
        let color;
        if fen_letter.to_lowercase() == fen_letter {
            color = Color::Black;
        } else {
            color = Color::White;
        }

        let piece_type = match fen_letter.to_lowercase().as_ref() {
            "k" => PieceType::King,
            "q" => PieceType::Queen,
            "p" => PieceType::Pawn,
            "n" => PieceType::Knight,
            "r" => PieceType::Rook,
            "b" => PieceType::Bishop,
        };

        Piece {
            piece_type: piece_type,
            color: color,
        }
    }

    pub fn to_fen(self) -> String {
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
}

pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub on_turn: Color,
}

impl Game {
    pub fn from_FEN(fen_code: String) -> Game {
        let splitted_fen = fen_code.split(" ");
        Game {
            board: [],
            on_turn: Color::White,
        }
    }
}
