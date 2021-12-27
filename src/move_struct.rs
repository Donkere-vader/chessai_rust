use crate::consts::{ MoveType, PieceType, Color };
use crate::piece::{ Piece };


#[derive(Copy, Clone)]
pub struct Move {
    pub from: [i8; 2],
    pub to: [i8; 2],
    pub piece: Option<Piece>,
}

impl Move {
    pub fn simple_new(from: [i8; 2], to: [i8; 2]) -> Move {
        Move {
            from: from,
            to: to,
            piece: None,
        }
    }

    pub fn from_long_algebraic_notatoin(notation: String) -> Move {
        let notation = notation.chars().collect::<Vec<char>>();
        let from_str = (notation[0], notation[1].to_digit(10).unwrap() as i8 - 1);
        let to_str = (notation[2], notation[3].to_digit(10).unwrap() as i8 - 1);

        let from: [i8; 2] = [
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|&x| x == from_str.0).unwrap() as i8,
            from_str.1,
        ];

        let to: [i8; 2]  = [
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().position(|&x| x == to_str.0).unwrap() as i8,
            to_str.1,
        ];


        let mut piece = None;
        if notation.len() > 4 {
            piece = Some(Piece::from_fen(notation[4]));
        }


        Move {
            from: from,
            to: to,
            piece: piece,
        }
    }

    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        format!("<Move {}>", self.long_algebraic_notation())
    }

    pub fn get_move_type(&self, castling: Option<Vec<Piece>>, en_passant_target_square: Option<[i8; 2]>, piece_type: Option<PieceType>) -> (MoveType, Option<Piece>) {
        if self.piece.is_some() {
            if self.from[1] == 6 || self.from[1] == 1 {
                return (MoveType::Promote, self.piece);
            }
        };

        if castling.is_some() && self.from[0] == 4 && (self.from[1] == 0 || self.from[1] == 7) {
            let castling = castling.unwrap();
            let color = if self.from[1] == 0 { Color::White } else { Color::Black };
            let king = Piece { piece_type: PieceType::King, color: color};
            let queen = Piece { piece_type: PieceType::Queen, color: color};
            if self.to[0] == 6 && castling.contains(&king) {
                return (MoveType::Castle, Some(king));
            } else if self.to[0] == 2 && castling.contains(&Piece { piece_type: PieceType::Queen, color: color}) {
                return (MoveType::Castle, Some(queen));
            }
        }

        if en_passant_target_square.is_some() && piece_type.is_some() && piece_type.unwrap() == PieceType::Pawn && en_passant_target_square.unwrap() == self.to {
            return (MoveType::EnPassant, None);
        }

        (MoveType::Standard, None)
    }

    pub fn long_algebraic_notation(&self) -> String {
        let from = format!("{}{}", (self.from[0] + 97) as u8 as char, self.from[1] + 1);
        let to = format!("{}{}", (self.to[0] + 97) as u8 as char, self.to[1] + 1);

        let promotion = match self.get_move_type(None, None, None).0 {
            MoveType::Promote => { 
                match self.piece {
                    Some(p) => {
                        match p.piece_type {
                            PieceType::King => "k",
                            PieceType::Queen => "q",
                            PieceType::Pawn => "p",
                            PieceType::Knight => "n",
                            PieceType::Rook => "r",
                            PieceType::Bishop => "b",
                        }
                    },
                    None => "",
                }
            },
            _ => "",
        };

        format!("{}{}{}", from, to, promotion)
    }
}
