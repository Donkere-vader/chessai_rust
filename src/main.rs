use std::io;
use std::thread;
use std::sync::mpsc::{ Receiver, self };
use consts::{ Command };
use logger::{ Logger, LogType };
use game::{ Game };

mod consts;
mod game;
mod piece;
mod piece_scores;
mod move_struct;
mod logger;


type ThreadMessage = (Command, String);


fn search_thread_func(thread_in: Receiver<ThreadMessage>) {
    loop {
        let (command, payload) = thread_in.recv().unwrap();

        match command {
            Command::Evaluate => {
                let mut game = Game::from_fen(payload);
                let best_move = game.get_best_move(5, false);
                println!("bestmove {}", best_move.long_algebraic_notation());
            },
        }
    }
}


fn main() {
    let logger = Logger::new("log.log");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut debug_mode = false;

    let (sender, reciever) = mpsc::channel::<ThreadMessage>();
    let _search_thread = thread::spawn(move || { search_thread_func(reciever) });
    let mut fen_string = String::new();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Couldn't read line");
        buffer = buffer.trim().to_string();
        logger.log(LogType::Info, format!("Recieved: {}", buffer));
        let splitted_buffer = buffer.split(" ").collect::<Vec<&str>>();
        let command = splitted_buffer[0];

        if command == "uci" {
            println!("id name Beth {}", env!("CARGO_PKG_VERSION"));
            println!("id author {}", env!("CARGO_PKG_AUTHORS"));
            println!("uciok");
        } else if command == "debug" {
            debug_mode = !debug_mode;
        } else if command == "isready" {
            println!("readyok");
        } else if command == "stop" {
            break;
        } else if command == "position" {
            fen_string = splitted_buffer[1].to_string();
            if fen_string == "startpos" {
                fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
            }
        } else if command == "go" {
            sender.send((
                Command::Evaluate,
                fen_string.to_string(),
            )).unwrap();
        }
    }
}
