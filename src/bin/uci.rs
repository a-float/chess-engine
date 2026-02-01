use std::io;
use std::io::Write;

use checkmatier::Board;
use checkmatier::evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator};
use checkmatier::search::{MinimaxSearch, SearchAlgorithm};

const ENGNINE_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/**
Based on https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html
*/
fn main() {
    println!("This is a UCI chess engine interface.");
    let mut input = String::new();
    let mut board = Board::default();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();
        let parts = command.split_whitespace().collect::<Vec<&str>>();

        match command {
            "quit" => break,
            "uci" => {
                println!("id name {} {}", capitalize(ENGNINE_NAME), VERSION);
                println!("id author {}", AUTHORS);
                println!("uciok");
            }
            "ucinewgame" => println!("readyok"),
            "isready" => println!("readyok"),
            _ if parts[0] == "position" => {
                board = match parts[1] {
                    "startpos" => Board::default(),
                    "fen" => {
                        let mut fen_str = String::new();
                        for part in &parts[2..] {
                            if part == &"moves" {
                                break;
                            }
                            fen_str.push_str(part);
                            fen_str.push(' ');
                        }
                        fen_str = fen_str.trim().to_string();
                        Board::from_fen(&fen_str)
                    }
                    _ => {
                        println!("Unrecognized position command");
                        continue;
                    }
                };
                let moves_index = parts.iter().position(|&x| x == "moves");
                if let Some(index) = moves_index {
                    for alg_move in &parts[index + 1..] {
                        println!("Applying move: {:?}", alg_move);
                        let m = board.get_move_from_algebraic_notation(alg_move);
                        if m.is_none() {
                            println!("Invalid move: {}", alg_move);
                            continue;
                        }
                        board.apply_move(&m.unwrap());
                    }
                }
            }
            "show" => println!("{}", board),
            _ if parts[0] == "go" => {
                let mut depth = 3;
                let evaluator = SumEvaluator::new(vec![
                    Box::new(MaterialEvaluator::new(10)),
                    Box::new(PositioningEvaluator::new(1)),
                ]);

                for i in (1..parts.len()).step_by(2) {
                    match parts[i] {
                        "depth" => {
                            if i + 1 < parts.len() {
                                depth = parts[i + 1].parse().unwrap();
                            }
                        }
                        _ => {
                            println!("Unrecognized go command: {}", parts[i]);
                            continue;
                        }
                    }
                }

                let best_move = MinimaxSearch::find_best_move(&board, &evaluator, depth);
                if let Some(mv) = best_move {
                    println!("bestmove {}", mv.to_long_algebraic_notation());
                } else {
                    println!("bestmove (none)");
                }
            }
            _ => println!("Unrecognized command"),
        }
    }
}
