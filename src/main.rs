mod board;
mod r#move;
use std::io;

use board::Board;

use crate::{board::piece::Color, r#move::Move};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    
    println!("{}", board);
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        println!("Your move (e.g. e4 e5):");
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(read) => {
                let line = buffer.trim();
                if line.is_empty() {
                    continue;
                }
                // println!("You typed: {} of size {}", line, read);
                if line == "q" {
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
