use std::io;
use std::thread;
use std::sync::mpsc::{ Receiver, self };
use std::sync::mpsc::TryRecvError;
use logger::{ Logger, LogType };
use game::{ Game };
use move_struct::{ Move };
use consts::{ Color };
use std::time::{ Duration };

mod consts;
mod game;
mod piece;
mod piece_scores;
mod move_struct;
mod logger;
mod utils;
mod openings;
#[cfg(test)]
mod tests;


fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer.len() != 0 {
            tx.send(buffer).unwrap();
        }
    });
    rx
}


fn main() {
    let logger = Logger::new("log.log");

    let mut debug_mode = false;
    let openings_database = openings::OpeningsDatabase::new();

    let mut search_thread: Option<thread::JoinHandle<()>> = None;
    #[allow(unused_assignments)]
    let (mut search_thread_sender, mut search_thread_receiver) = mpsc::channel::<Move>();
    let mut game: Game = Game::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    let mut got_initial_position = false;
    let stdin_channel = spawn_stdin_channel();

    'main: loop {
        match stdin_channel.try_recv() {
            Ok(message) => {
                logger.log(LogType::Info, format!("Recieved: {}", message));
                let splitted_buffer = message.split(" ").collect::<Vec<&str>>();
                let command = splitted_buffer[0];

                if command == "uci" {
                    println!("id name Beth {}", env!("CARGO_PKG_VERSION"));
                    println!("id author {}", env!("CARGO_PKG_AUTHORS"));
                    println!("uciok");
                } else if command == "debug" {
                    debug_mode = !debug_mode;
                } else if command == "isready" {
                    println!("readyok");
                } else if command == "quit" {
                    break 'main;
                } else if command == "position" {
                    let mut start_fen_string = splitted_buffer[1].to_string();
                    if start_fen_string == "startpos" {
                        start_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
                    }

                    let mut looking_at_move = false;
                    let mut moves = Vec::new();
                    game = Game::from_fen(start_fen_string);
                    for item in splitted_buffer.iter() {
                        if looking_at_move {
                            let m = Move::from_long_algebraic_notatoin(String::from(*item));
                            moves.push(m);
                        } else if *item == "moves" {
                            looking_at_move = true
                        }
                    }

                    game.apply_moves(&moves);
                    got_initial_position = true;

                    let mut board_text = String::from("Board:\n");
                    for y in 0..8 {
                        for x in 0..8 {
                            if game.board[7 - y][x].is_some() {
                                board_text += &game.board[7 - y][x].unwrap().to_fen();
                            } else {
                                board_text += " ";
                            }
                        }
                        board_text += "\n";
                    }
                    logger.log(LogType::Info, board_text);
                } else if command == "go" && got_initial_position {
                    game.show_board(None, Color::Black);
                    println!("FENCODE: {}", game.to_fen());

                    let game_clone = game.clone();
                    let thread_communicators = mpsc::channel::<Move>();
                    search_thread_sender = thread_communicators.0;
                    search_thread_receiver = thread_communicators.1;

                    let odb = openings_database.clone();
                    search_thread = Some(thread::spawn(move || {
                        let best_move = game_clone.get_best_move(5, &odb);
                        search_thread_sender.send(best_move).unwrap();
                    }));
                } else if command == "stop" {
                    if search_thread.is_some() {

                    }
                } else {
                    logger.log(LogType::Warn, format!("Couldn't handle command `{}` at this time", command));
                }
            },
            Err(TryRecvError::Empty) => {
                match &search_thread {
                    Some(_) => {
                        match search_thread_receiver.try_recv() {
                            Ok(mve) => {
                                game.do_move(&mve);

                                let mut board_text = String::from("Board:\n");
                                for y in 0..8 {
                                    for x in 0..8 {
                                        if game.board[7 - y][x].is_some() {
                                            board_text += &game.board[7 - y][x].unwrap().to_fen();
                                        } else {
                                            board_text += " ";
                                        }
                                    }
                                    board_text += "\n";
                                }
                                logger.log(LogType::Info, board_text);

                                logger.log(LogType::Info, mve.repr());

                                println!("bestmove {}", mve.long_algebraic_notation());
                                search_thread = None;
                            },
                            Err(TryRecvError::Empty) => {},
                            Err(TryRecvError::Disconnected) => panic!("Search thread connection lost"),
                        }
                    },
                    None => {},
                }
            },
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        thread::sleep(Duration::from_millis(100));
    }
}
