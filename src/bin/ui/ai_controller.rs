use super::app::App;
use checkmatier::search::{MinimaxSearch, SearchAlgorithm};
use std::time::Instant;

impl App {
    pub fn toggle_ai(&mut self) {
        self.ai_enabled = !self.ai_enabled;
    }

    pub fn toggle_ai_color(&mut self) {
        self.ai_color = self.ai_color.opposite();
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
        let start = Instant::now();
        if let Some(best_move) =
            MinimaxSearch::find_best_move(&self.board, &self.ai_evaluator, self.ai_depth)
        {
            self.ai_last_move_time = Some(start.elapsed());
            self.board.apply_move(&best_move);
            self.move_history.push(best_move);
            self.active_square = None;
            self.possible_moves.clear();
        }
    }

    pub fn check_and_make_ai_move(&mut self) {
        if self.ai_enabled && self.board.get_active_color() == self.ai_color {
            self.make_ai_move();
        }
    }
}
