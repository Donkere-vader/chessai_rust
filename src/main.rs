mod storage;
mod consts;

use consts::{ Piece, PieceType, Color };

fn main() {
    let piece = Piece { piece_type: PieceType::King, color: Color::White };
    println!("{}", piece.repr());
    println!("{}", storage::loads());
}
