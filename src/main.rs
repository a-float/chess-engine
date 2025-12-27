mod board;
mod r#move;
mod search;
use board::Board;
use std::io;

use crate::{board::piece::Color, search::Search};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);

    println!("{}", board);
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        if !board.is_white_turn {
            let m = Search::random_move(&board);
            if m.is_none() {
                println!("Black has no moves: White wins");
                break;
            }
            board.apply_move(&m.unwrap());
            println!("\n{}", board);
            continue;
        }
        println!("Your move (e.g. e2 e4):");
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(_read) => {
                let line = buffer.trim();
                if line.is_empty() {
                    continue;
                }
                // println!("You typed: {} of size {}", line, read);
                if line == ":q" {
                    println!("Quitting");
                    break;
                }
                if let Some(m) = board.get_legal_move_from_string(&line)
                    && m.piece.get_color() == Color::White
                {
                    board.apply_move(&m);
                    println!("\n{}", board);
                    println!("Your last move {}", m);
                } else {
                    println!("Invalid move: {}", line);
                }
            }
        }
    }
}
