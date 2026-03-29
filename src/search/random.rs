use super::{SearchAlgorithm, SearchInfo, SearchLimits};
use crate::{Board, evaluate::Evaluator, r#move::Move};
use rand::{rng, seq::IndexedRandom};
use std::sync::{Arc, atomic::AtomicBool};

pub struct RandomSearch {}

impl SearchAlgorithm for RandomSearch {
    fn search(
        &mut self,
        board: &Board,
        _evaluator: Arc<dyn Evaluator>,
        _limits: SearchLimits,
        _stop_flag: Arc<AtomicBool>,
        _info_callback: Option<Box<dyn Fn(SearchInfo) + Send>>,
    ) -> Option<Move> {
        let mut rng = rng();
        let moves = board.get_legal_moves();
        moves.choose(&mut rng).copied()
    }
}
