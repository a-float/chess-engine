mod minimax;
mod random;

use crate::{board::Board, evaluate::Evaluator, r#move::Move};
pub use minimax::MinimaxSearch;
pub use random::RandomSearch;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct SearchLimits {
    pub max_depth: Option<u8>,
    pub max_time: Option<Duration>,
    pub max_nodes: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SearchInfo {
    pub depth: u8,
    pub score: i32,
    pub nodes: usize,
    pub time: Duration,
    pub pv: Vec<Move>, // Principal variation (best line)
}

pub fn should_stop(
    limits: &SearchLimits,
    start_time: &Instant,
    nodes_searched: usize,
    stop_flag: &Arc<AtomicBool>,
) -> bool {
    if stop_flag.load(Ordering::Relaxed) {
        return true;
    }

    if let Some(max_time) = limits.max_time {
        if start_time.elapsed() >= max_time {
            return true;
        }
    }

    if let Some(max_nodes) = limits.max_nodes {
        if nodes_searched >= max_nodes {
            return true;
        }
    }

    false
}

pub trait SearchAlgorithm {
    fn search(
        &mut self,
        board: &Board,
        evaluator: Arc<dyn Evaluator>,
        limits: SearchLimits,
        stop_flag: Arc<AtomicBool>,
        info_callback: Option<Box<dyn Fn(SearchInfo) + Send>>,
    ) -> Option<Move>;

    fn search_simple(
        &mut self,
        board: &Board,
        evaluator: Arc<dyn Evaluator>,
        limits: SearchLimits,
    ) -> Option<Move> {
        self.search(
            board,
            evaluator,
            limits,
            Arc::new(AtomicBool::new(false)),
            None,
        )
    }
}
