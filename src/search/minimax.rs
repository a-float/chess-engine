use std::cmp;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;

use super::{SearchAlgorithm, SearchInfo, SearchLimits};
use crate::{Board, evaluate::Evaluator, r#move::Move};

pub struct MinimaxSearch {
    nodes_searched: usize,
}

impl MinimaxSearch {
    pub fn new() -> Self {
        Self { nodes_searched: 0 }
    }

    fn minimax(
        &mut self,
        board: &mut Board,
        evaluator: &Arc<dyn Evaluator>,
        depth: u8,
        stop_flag: &Arc<AtomicBool>,
    ) -> Option<i32> {
        if stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        self.nodes_searched += 1;

        if depth == 0 {
            return Some(evaluator.evaluate(board));
        }

        let moves = board.get_legal_moves();
        if moves.is_empty() {
            return Some(if board.is_in_check() { -100000 } else { 0 });
        }

        let mut best_score = i32::MIN;
        for mv in moves {
            board.apply_move(&mv);
            let score = match self.minimax(board, evaluator, depth - 1, stop_flag) {
                Some(s) => -s,
                None => {
                    board.undo_move(&mv);
                    return None; // Propagate stop signal
                }
            };
            board.undo_move(&mv);

            best_score = cmp::max(best_score, score);
        }

        Some(best_score)
    }
}

impl SearchAlgorithm for MinimaxSearch {
    fn search(
        &mut self,
        board: &Board,
        evaluator: Arc<dyn Evaluator>,
        limits: SearchLimits,
        stop_flag: Arc<AtomicBool>,
        info_callback: Option<Box<dyn Fn(SearchInfo) + Send>>,
    ) -> Option<Move> {
        let start_time = Instant::now();
        self.nodes_searched = 0;

        let moves = board.get_legal_moves();
        if moves.is_empty() {
            return None;
        }

        let mut is_stopped = false;
        let mut board_clone = board.clone();
        let mut best_move = moves[0];
        let mut best_score;
        let mut current_depth = 1;

        // Iterative deepening
        while !is_stopped && current_depth <= limits.max_depth.unwrap_or(u8::MAX) {
            let mut depth_best_move = moves[0];
            let mut depth_best_score = i32::MIN;

            for mv in &moves {
                board_clone.apply_move(mv);
                let score =
                    match self.minimax(&mut board_clone, &evaluator, current_depth - 1, &stop_flag)
                    {
                        Some(s) => -s,
                        None => {
                            board_clone.undo_move(mv);
                            break;
                        }
                    };
                board_clone.undo_move(mv);

                if score > depth_best_score {
                    depth_best_score = score;
                    depth_best_move = *mv;
                }

                // TODO consider adding this check inside minimax to be more accurate
                if super::should_stop(&limits, &start_time, self.nodes_searched, &stop_flag) {
                    is_stopped = true;
                    break;
                }
            }

            // Update best move from completed depth
            if !stop_flag.load(Ordering::Relaxed) {
                best_move = depth_best_move;
                best_score = depth_best_score;

                // Send info update
                if let Some(ref callback) = info_callback {
                    callback(SearchInfo {
                        depth: current_depth,
                        score: best_score,
                        nodes: self.nodes_searched,
                        time: start_time.elapsed(),
                        pv: vec![best_move], // Collect PV?
                    });
                }
            }

            current_depth += 1;
        }

        Some(best_move)
    }
}

impl Default for MinimaxSearch {
    fn default() -> Self {
        Self::new()
    }
}
