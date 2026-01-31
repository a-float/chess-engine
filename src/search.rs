mod minimax;
mod random;

use crate::{board::Board, evaluate::Evaluator, r#move::Move};
pub use minimax::MinimaxSearch;
pub use random::RandomSearch;

pub trait SearchAlgorithm {
    fn find_best_move(board: &Board, evaluator: &dyn Evaluator, depth: u8) -> Option<Move>;
}
