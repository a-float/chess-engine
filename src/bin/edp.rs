use clap::Parser;
use std::{
    fs,
    io::{Write, stdout},
    sync::Arc,
};

use checkmatier::{
    Board,
    edp::EDP,
    evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator},
    r#move::Move,
    search::{MinimaxSearch, SearchAlgorithm, SearchLimits},
};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TestResult {
    #[tabled(rename = "Best found")]
    best_found: String,
    #[tabled(rename = "Best move")]
    best_move: String,
    #[tabled(rename = "To avoid")]
    to_avoid: String,
    #[tabled(rename = "Id")]
    id: String,
    #[tabled(rename = "Correct")]
    correct: bool,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = EIGENMAN)]
    edp_file: String,
    #[arg(short, long)]
    show_idx: Option<u32>,
}

const EIGENMAN: &str = "tests/eigenman-rapid-engine-test.txt";
const WAC: &str = "tests/wac.txt";

fn main() {
    let args = Args::parse();
    let path = match args.edp_file.as_str() {
        "eigenman" => EIGENMAN,
        "wac" => WAC,
        _ => panic!("Unknown EDP file: {}", args.edp_file),
    };
    let test_suite = fs::read_to_string(path).expect("Failed to read EDP test suite");

    if let Some(idx) = args.show_idx {
        let line = test_suite
            .lines()
            .nth(idx as usize)
            .expect("Index out of bounds");
        debug_test_case(line);
    } else {
        run_all(test_suite);
    }
}

fn debug_test_case(edp_str: &str) {
    let (board, ops) = EDP::from_str(edp_str);
    let bm = solve(&board);
    println!("----------------------------------------------------");
    println!("{}", board);
    println!("----------------------------------------------------");
    println!("Position ID: {}", ops.id.unwrap_or("?".to_string()));
    println!("FEN: {}", board.to_fen());
    println!(
        "Found best move: {}",
        bm.unwrap().to_short_algebraic_notation(&board)
    );
    println!("Actual best move: {}", ops.bm.unwrap_or("?".to_string()));
    println!("Moves to avoid: {}", ops.am.unwrap_or("?".to_string()));
    println!("----------------------------------------------------");
}

fn solve(board: &Board) -> Option<Move> {
    let mut search = MinimaxSearch::new();
    let evaluator = Arc::new(SumEvaluator::new(vec![
        Box::new(MaterialEvaluator::new(10)),
        Box::new(PositioningEvaluator::new(1)),
    ]));

    let search_limits = SearchLimits {
        max_depth: Some(3),
        ..Default::default()
    };

    search.search_simple(&board, evaluator, search_limits)
}

fn run_all(test_suite: String) -> () {
    let mut correct = 0;
    let mut results = Vec::new();
    let total = test_suite.lines().count();
    let time_start = std::time::Instant::now();

    println!("\nRunning EDP test suite with {} positions", total);

    for (i, line) in test_suite.lines().enumerate() {
        let (board, ops) = EDP::from_str(line);

        let bm = solve(&board);

        let my_bm_algebraic = bm.unwrap().to_short_algebraic_notation(&board);
        let bm_algebraic = ops.bm.unwrap_or("?".to_string());
        let is_correct = bm_algebraic == my_bm_algebraic;

        if is_correct {
            correct += 1;
        }

        results.push(TestResult {
            best_found: my_bm_algebraic,
            best_move: bm_algebraic,
            to_avoid: ops.am.unwrap_or("?".to_string()),
            id: ops.id.unwrap_or("?".to_string()),
            correct: is_correct,
        });

        print!("\rProcessed {}/{} positions", i + 1, total);
        stdout().flush().unwrap();
    }

    print!("\r{:<50}\r", "");
    println!();

    let time_end = std::time::Instant::now();

    println!("{}", Table::new(results));
    println!("\nCorrect: {}/{}", correct, total);
    println!("Time taken: {:.2?}\n", time_end - time_start);
}
