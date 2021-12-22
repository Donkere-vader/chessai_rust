use crate::consts::{ PieceType, Color, Move, MoveType };
use crate::piece_scores::{ SCORE_KING, SCORE_QUEEN, SCORE_ROOK, SCORE_BISHOP, SCORE_KNIGHT, SCORE_PAWN };
use crate::game::{ Game };


#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}


impl Piece {
    pub fn get_all_moves(&self, x: i8, y: i8, game: &Game) -> Vec<Move> {
        fn walk_offsets(piece: &Piece, from: [i8; 2], board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, max_distance: Option<u32>, take: bool) -> Vec<Move> {
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

                    // Chech if tile is empty or takable
                    match &board[current_coord[1] as usize][current_coord[0] as usize] {
                        Some(p) => {
                            if p.color != piece.color && take {
                                new_moves.push( Move::simple_new(*&from, *&current_coord) );
                            }
                            break;
                        },
                        None => new_moves.push( Move::simple_new(*&from, *&current_coord) ),
                    }

                    // Chech if max_distance is not yet reached
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

        fn with_offsets(piece: &Piece, from: [i8; 2], board: [[Option<Piece>; 8]; 8], offsets: Vec<[i8; 2]>, has_to_take: bool) -> Vec<Move> {
            let mut new_moves: Vec<Move> = Vec::new();

            for offset in offsets {
                let new_coord = [from[0] + offset[0], from[1] + offset[1]];

                // Check if new_coord exists
                if !(0 <= new_coord[0] && new_coord[0] <= 7) || !(0 <= new_coord[1] && new_coord[1] <= 7) {
                    continue;
                }

                // Check if tile is empty or takable
                match &board[new_coord[1] as usize][new_coord[0] as usize] {
                    Some(p) => if p.color != piece.color { new_moves.push( Move::simple_new(*&from, *&new_coord) ); },
                    None => if !has_to_take { new_moves.push( Move::simple_new(*&from, *&new_coord) ) },
                }
            }

            new_moves
        }

        let mut moves: Vec<Move> = Vec::new();
        
        match self.piece_type {
            PieceType::King => {
                // standard moves
                moves.extend(with_offsets(self, [x, y], game.board, vec![[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]], false));

                // castle moves
                for (y, color) in vec![(0, Color::White), (7, Color::Black)] {
                    if self.color == color {
                        let king = Piece { piece_type: PieceType::King, color: color};
                        let queen = Piece { piece_type: PieceType::Queen, color: color};
                        if game.castle.contains(&king) &&
                            game.board[y][5].is_none() && game.board[y][6].is_none() {
                                moves.push(Move { from: [x, y as i8], to: [7, y as i8], move_type: MoveType::Castle, piece: Some(king)});
                        }
                        if game.castle.contains(&queen) &&
                            game.board[y][1].is_none() && game.board[y][2].is_none() && game.board[y][3].is_none() {
                                moves.push(Move { from: [x, y as i8], to: [0, y as i8], move_type: MoveType::Castle, piece: Some(queen)});
                        }
                    }
                }
            },
            PieceType::Queen => moves.extend(walk_offsets(self, [x, y], game.board, vec![[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]], None, true)),
            PieceType::Bishop => moves.extend(walk_offsets(self, [x, y], game.board, vec![[1, 1], [-1, 1], [-1, -1], [1, -1]], None, true)),
            PieceType::Knight => moves.extend(with_offsets(self, [x, y], game.board, vec![[-1, 2], [1, 2], [2, 1], [2, -1], [1, -2], [-1, -2], [-2, -1], [-2, 1]], false)),
            PieceType::Rook => moves.extend(walk_offsets(self, [x, y], game.board, vec![[1, 0], [0, 1], [-1, 0], [0, -1]], None, true)),
            PieceType::Pawn => {
                // can take pieces?
                moves.extend(with_offsets(self, [x, y], game.board, if self.color == Color::White { vec![[1, 1], [-1, 1]] } else { vec![[1, -1], [-1, -1]] }, true));

                if (self.color == Color::White && y == 6 && game.board[7][x as usize].is_none() ) || (self.color == Color::Black && y == 1 && game.board[0][x as usize].is_none()) {
                    // promote
                    let to_y = if self.color == Color::White { 7 } else { 0 };
                    for piece_type in vec![PieceType::Queen, PieceType::Knight, PieceType::Rook, PieceType::Bishop] {
                        moves.push(Move { from: [x, y], to: [x, to_y], move_type: MoveType::Promote, piece: Some(Piece { piece_type: piece_type, color: self.color }) } );
                    }
                } else {
                    // standard moves
                    moves.extend(walk_offsets(
                        self,
                        [x, y],
                        game.board,
                        if self.color == Color::White { vec![[0, 1]] } else { vec![[0, -1]] },
                        if (self.color == Color::White && y == 1) || (self.color == Color::Black && y == 6) { Some(2) } else { Some(1) },
                        false,
                    ));
                }
            },
        }
        moves
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
            "b" => PieceType::Bishop,
            _ => panic!("Invalid char `{}`", fen_letter),
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

    pub fn score(&self, x: usize, mut y: usize) -> i64 {
        if self.color == Color::Black {
            y = 7 - y;
        }

        match self.piece_type {
            PieceType::King => SCORE_KING[y][x],
            PieceType::Queen => SCORE_QUEEN[y][x] * 9,
            PieceType::Bishop => SCORE_BISHOP[y][x] * 3,
            PieceType::Knight => SCORE_KNIGHT[y][x] * 3,
            PieceType::Rook => SCORE_ROOK[y][x] * 5,
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
            PieceType::Pawn => String::from("♟"),
        }
    }

    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        format!("<Piece {:?} {:?}>", self.piece_type, self.color)
    }
}
