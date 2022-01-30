
use crate::game::{ Game };
use std::time::{ SystemTime };
use crate::openings::{ OpeningsDatabase };
use crate::consts::{ SEARCH_DEPTH };
use std::io;
use std::io::Write;
use std::fs::{ File };
use std::path::{ Path };


pub fn run_benchmarks(save_file: Option<String>) {
    if save_file.is_some() {
        if Path::new(save_file.as_ref().unwrap()).exists() {
            panic!("Output file already exists")
        }
    }

    println!("[ === Starting benchmarks === ]\n");
    let opening_db = OpeningsDatabase::new();

    let fen_codes = vec![
        ("start_code", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0".to_string()),
        ("mate_in_2_rooks", "7k/7r/5r2/K7/8/8/8/8 b KQkq - 0 1".to_string()),
        ("promote_for_black", "8/1k6/8/8/8/8/7p/K7 w KQkq - 0 1".to_string()),
        ("game_against_serochess", "r2qkbnr/2pNpppp/p1Q5/8/3p2b1/2P5/PP1PPP1P/RNB1KB1R w KQkq - 0 9".to_string()),
        ("castle_black", "4k2r/8/8/8/8/8/8/4KR2 b k - 0 1".to_string())
    ];

    let mut results = Vec::new();

    for (idx, fen_code) in fen_codes.iter().enumerate() {
        let game = Game::from_fen(fen_code.1.to_string());
        let mut recorded_times = Vec::new();

        print!("{: >2}/{: <2} {: <30}", idx + 1, fen_codes.len(), fen_code.0);
        io::stdout().flush().unwrap();

        for _ in 0..10 {
            let start = SystemTime::now();
            game.get_best_move(SEARCH_DEPTH, &opening_db);
            recorded_times.push(start.elapsed().unwrap().as_nanos());
            print!("|");
            io::stdout().flush().unwrap();
        }

        println!();

        let mut total = 0;
        for recorded_time in recorded_times.iter() {
            total += *recorded_time;
        }
        let average = total / recorded_times.len() as u128;
        results.push((fen_code.0, average));
    }

    println!("\nRESULTS:");

    for res in results.iter() {
        println!("{: <30} {}ns", res.0, res.1);
    }

    if save_file.is_some() {
        let mut file = File::create(&save_file.unwrap()).unwrap();
        for res in results.iter() {
            file.write_all(&format!("{},{}\n", res.0, res.1).as_bytes()).unwrap();
        }
    }
}
