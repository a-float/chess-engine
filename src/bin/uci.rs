use std::io;
use std::io::Write;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

use checkmatier::Board;
use checkmatier::evaluate::{Evaluator, MaterialEvaluator, PositioningEvaluator, SumEvaluator};
use checkmatier::search::{MinimaxSearch, SearchAlgorithm, SearchInfo, SearchLimits};

const ENGINE_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

struct UciEngine {
    board: Board,
    #[allow(dead_code)]
    search: MinimaxSearch,
    stop_flag: Arc<AtomicBool>,
}

impl UciEngine {
    fn new() -> Self {
        Self {
            board: Board::default(),
            search: MinimaxSearch::new(),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    fn handle_position(&mut self, parts: &[&str]) {
        if parts.len() > 1 && parts[1] == "startpos" {
            self.board = Board::default();
        } else if parts.len() > 2 && parts[1] == "fen" {
            let fen_end = parts
                .iter()
                .position(|&s| s == "moves")
                .unwrap_or(parts.len());
            let fen = parts[2..fen_end].join(" ");
            self.board = Board::from_fen(&fen);
        } else {
            eprintln!("Invalid position command: {}", parts.join(" "));
            return;
        }

        if let Some(moves_idx) = parts.iter().position(|&s| s == "moves") {
            for move_str in &parts[moves_idx + 1..] {
                if let Some(mv) = self.board.get_move_from_algebraic_notation(move_str) {
                    self.board.apply_move(&mv);
                } else {
                    eprintln!("Invalid move: {}", move_str);
                }
            }
        }
    }

    fn handle_go(&mut self, parts: &[&str]) {
        self.stop_flag.store(false, Ordering::Relaxed);

        let mut limits = SearchLimits {
            max_depth: Some(10),
            max_time: None,
            max_nodes: None,
        };

        let mut i = 1;
        while i < parts.len() {
            match parts[i] {
                "depth" if i + 1 < parts.len() => {
                    limits.max_depth = parts[i + 1].parse().ok();
                    i += 2;
                }
                "movetime" if i + 1 < parts.len() => {
                    if let Ok(ms) = parts[i + 1].parse::<u64>() {
                        limits.max_time = Some(Duration::from_millis(ms));
                    }
                    i += 2;
                }
                "nodes" if i + 1 < parts.len() => {
                    limits.max_nodes = parts[i + 1].parse().ok();
                    i += 2;
                }
                "infinite" => {
                    limits.max_depth = None;
                    limits.max_time = None;
                    i += 1;
                }
                _ => i += 1,
            }
        }

        println!("Limits {:?}", limits);
        let board = self.board.clone();
        let stop_flag = self.stop_flag.clone();

        thread::spawn(move || {
            let mut search = MinimaxSearch::new();
            let evaluator: Arc<dyn Evaluator> = Arc::new(SumEvaluator::new(vec![
                Box::new(MaterialEvaluator::new(2)),
                Box::new(PositioningEvaluator::new(1)),
            ]));

            let info_callback = Box::new(|info: SearchInfo| {
                println!(
                    "info depth {} score cp {} nodes {} time {} pv {}",
                    info.depth,
                    info.score,
                    info.nodes,
                    info.time.as_millis(),
                    info.pv
                        .iter()
                        .map(|m| m.to_long_algebraic_notation())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            });

            if let Some(best_move) =
                search.search(&board, evaluator, limits, stop_flag, Some(info_callback))
            {
                println!("bestmove {}", best_move.to_long_algebraic_notation());
            } else {
                println!("bestmove (none)");
            }
        });
    }

    fn handle_stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}

/**
Based on https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html
*/
fn main() {
    println!("{} {} made by Mati", capitalize(ENGINE_NAME), VERSION);

    let mut engine = UciEngine::new();
    let mut input = String::new();

    loop {
        eprint!("> ");
        io::stderr().flush().unwrap();
        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "quit" => break,
            "uci" => {
                println!("id name {} {}", capitalize(ENGINE_NAME), VERSION);
                println!("id author {}", AUTHORS);
                println!("uciok");
            }
            "debug" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "on" => eprintln!("Debug mode enabled"),
                        "off" => eprintln!("Debug mode disabled"),
                        _ => {}
                    }
                }
            }
            "isready" => println!("readyok"),
            "setoption" => {}
            "ucinewgame" => {
                engine = UciEngine::new();
            }
            "position" => engine.handle_position(&parts),
            "go" => engine.handle_go(&parts),
            "stop" => engine.handle_stop(),
            "show" => println!("{}", engine.board),
            _ => {
                eprintln!("Unrecognized command: {}", command);
            }
        }
    }
}
