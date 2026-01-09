use std::{env, time::SystemTime};

use chess_engine::Board;

fn perft(depth: u8, board: &mut Board) -> usize {
    let moves = board.get_moves();
    match depth {
        0 => 1,
        1 => moves.len(),
        _ => {
            let mut sum = 0;
            for m in moves {
                board.apply_move(&m);
                sum += perft(depth - 1, board);
                board.undo_move(&m);
            }
            sum
        }
    }
}

static EXPECTED: [usize; 9] = [1, 20, 400, 8902, 197_281, 4_865_609, 119_060_324, 3_195_901_860, 84_998_978_956];

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_depth: u8 = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .expect("Please provide a valid depth argument");

    let mut board = Board::default();

    println!("| Depth | Nodes      | Time (ms) | Expected   | Difference  |");
    println!("|:-----:|-----------:|----------:|-----------:|------------:|");

    for depth in 1..=max_depth {
        let start = SystemTime::now();
        let positions = perft(depth, &mut board);
        let end = SystemTime::now();
        let elapsed = end.duration_since(start).unwrap().as_millis();

        let expected = if (depth as usize) < EXPECTED.len() {
            EXPECTED[depth as usize]
        } else {
            0
        };
        let difference = positions as i64 - expected as i64;

        println!(
            "| {:5} | {:10} | {:9} | {:10} | {:+11} |",
            depth, positions, elapsed, expected, difference
        );
    }
}
