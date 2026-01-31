use crate::Board;
use crate::board::piece::{Color, PieceKind};
use crate::board::square::Square;

use super::Evaluator;

pub struct MaterialEvaluator {}

impl MaterialEvaluator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Evaluator for MaterialEvaluator {
    fn name(&self) -> String {
        "material".to_string()
    }
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;
        for idx in 0..64 {
            if let Some(piece) = board.get_piece(Square::from_index(idx).unwrap()) {
                let piece_value = match piece.get_kind() {
                    PieceKind::Pawn => 100,
                    PieceKind::Knight => 320,
                    PieceKind::Bishop => 330,
                    PieceKind::Rook => 500,
                    PieceKind::Queen => 900,
                    PieceKind::King => 20000,
                };
                if piece.get_color() == Color::White {
                    score += piece_value;
                } else {
                    score -= piece_value;
                }
            }
        }
        score * board.get_active_color().get_value()
    }
}
