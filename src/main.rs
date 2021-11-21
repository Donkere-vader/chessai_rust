mod consts;
mod game;
mod piece;
mod piece_scores;


use consts::{ PieceType, Color, Move };
use game::{ Game };


fn main() {
    // let game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    // game.show_board(None);
    // println!("{}", match &game.board[0][0] {
    //     Some(p) => p.repr(),
    //     _ => String::from("None"),
    // });

    let game = Game::from_fen(String::from("8/8/8/8/8/4P3/8/8 w KQkq - 0 1"));
    game.show_board(None);
    let all_moves = game.get_all_moves(Color::White);

    println!("\nALL MOVES:");
    let mut highlight: Vec<[i8; 2]> = Vec::new();
    for m in all_moves {
        println!("{}", m.repr());
        highlight.push(m.to);
    }

    game.show_board(Some(highlight));
}
