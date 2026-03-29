use super::app::App;
use checkmatier::search::{MinimaxSearch, SearchAlgorithm, SearchLimits};
use std::sync::mpsc::Sender;
use std::sync::{Arc, atomic::AtomicBool};
use std::thread;
use std::time::Instant;

impl App {
    pub fn toggle_ai(&mut self) {
        self.ai_enabled = !self.ai_enabled;
    }

    pub fn toggle_ai_color(&mut self) {
        self.ai_color = self.ai_color.opposite();
        if self.board.get_active_color() == self.ai_color {
            self.make_ai_move();
        }
    }

    pub fn increase_ai_depth(&mut self) {
        if self.ai_depth < 10 {
            self.ai_depth += 1;
        }
    }

    pub fn decrease_ai_depth(&mut self) {
        if self.ai_depth > 1 {
            self.ai_depth -= 1;
        }
    }

    pub fn make_ai_move(&mut self) {
        // Start AI search in background and return immediately.
        if self.ai_searching {
            return;
        }

        self.ai_searching = true;
        self.ai_last_start_move_time = Some(Instant::now());
        let tx: Sender<Option<checkmatier::r#move::Move>> = self.ai_move_tx.clone();
        let board = self.board.clone();
        let evaluator = Arc::clone(&self.ai_evaluator);
        let depth = self.ai_depth;

        thread::spawn(move || {
            let mut search = MinimaxSearch::new();
            let stop_flag = Arc::new(AtomicBool::new(false));
            let limits = SearchLimits {
                max_depth: Some(depth),
                max_time: None,
                max_nodes: None,
            };

            let mv = search.search(&board, evaluator, limits, stop_flag, None);
            let _ = tx.send(mv);
        });
    }

    /// Called every UI tick to apply any completed AI move.
    pub fn poll_ai_move(&mut self) {
        if let Ok(opt_mv) = self.ai_move_rx.try_recv() {
            self.ai_searching = false;
            if let Some(best_move) = opt_mv {
                self.board.apply_move(&best_move);
                self.move_history.push(best_move);
                self.active_square = None;
                self.possible_moves.clear();
                self.ai_last_end_move_time = Some(Instant::now());
            }
        }
    }

    pub fn check_and_make_ai_move(&mut self) {
        if self.ai_enabled && self.board.get_active_color() == self.ai_color {
            self.make_ai_move();
        }
    }
}
