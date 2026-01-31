use super::Evaluator;
use crate::{Board, board::piece::Color};

pub struct MobilityEvaluator {
    weight: i32,
}

impl MobilityEvaluator {
    pub fn new(weight: i32) -> Self {
        Self { weight }
    }
}

impl Evaluator for MobilityEvaluator {
    fn name(&self) -> String {
        "mobility".to_string()
    }
    /**
    Use with caution. Legal move calculation is currently very compute expensive.
    */
    fn evaluate(&self, board: &Board) -> i32 {
        let white_moves = board.get_legal_moves_for_color(Color::White);
        let black_moves = board.get_legal_moves_for_color(Color::Black);
        (white_moves.len() as i32 - black_moves.len() as i32)
            * self.weight
            * board.get_active_color().get_value()
    }
}
