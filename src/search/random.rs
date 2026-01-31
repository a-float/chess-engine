use super::SearchAlgorithm;
use crate::{Board, evaluate::Evaluator, r#move::Move};
use rand::{rng, seq::IndexedRandom};

pub struct RandomSearch {}

impl SearchAlgorithm for RandomSearch {
    fn find_best_move(board: &Board, _evaluator: &dyn Evaluator, _depth: u8) -> Option<Move> {
        let mut rng = rng();
        let moves = board.get_legal_moves();
        moves.choose(&mut rng).copied()
    }
}
