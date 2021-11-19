mod consts;
mod game;
mod piece;
mod piece_scores;


use consts::{ PieceType, Color };
use game::{ Game };


fn main() {
    let game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    game.show_board();
    println!("{}\n{}", game.to_fen(), game.get_board_score(Color::White));
}
