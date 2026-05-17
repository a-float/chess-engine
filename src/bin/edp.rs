use std::{
    fs,
    io::{Write, stdout},
    sync::Arc,
};

use checkmatier::{
    edp::EDP,
    evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator},
    search::{MinimaxSearch, SearchAlgorithm, SearchLimits},
};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TestResult {
    #[tabled(rename = "Best move")]
    best_move: String,
    #[tabled(rename = "Best found")]
    best_found: String,
    #[tabled(rename = "To avoid")]
    to_avoid: String,
    #[tabled(rename = "Id")]
    id: String,
}

const EIGENMAN: &str = "tests/eigenman-rapid-engine-test.txt";
const WAC: &str = "tests/wac.txt";

fn main() {
    let test_suite = fs::read_to_string(EIGENMAN).expect("Failed to read EDP test suite");

    let total = test_suite.lines().count();
    let time_start = std::time::Instant::now();
    let mut correct = 0;
    let mut results = Vec::new();

    println!("\nRunning EDP test suite with {} positions", total);

    for (i, line) in test_suite.lines().enumerate() {
        let (board, ops) = EDP::from_str(line);

        let mut search = MinimaxSearch::new();
        let evaluator = Arc::new(SumEvaluator::new(vec![
            Box::new(MaterialEvaluator::new(10)),
            Box::new(PositioningEvaluator::new(1)),
        ]));

        let search_limits = SearchLimits {
            max_depth: Some(3),
            ..Default::default()
        };

        let bm = search.search_simple(&board, evaluator, search_limits);

        if bm.and_then(|m| Some(m.to_long_algebraic_notation())) == ops.bm {
            correct += 1;
        }

        results.push(TestResult {
            best_move: bm.unwrap().to_long_algebraic_notation(),
            best_found: ops.bm.unwrap_or("?".to_string()),
            to_avoid: ops.am.unwrap_or("?".to_string()),
            id: ops.id.unwrap_or("?".to_string()),
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
