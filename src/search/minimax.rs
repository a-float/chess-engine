use super::SearchAlgorithm;
use crate::{Board, board::piece::Color, evaluate::Evaluator, r#move::Move};

pub struct MinimaxSearch {}

impl MinimaxSearch {
    fn minimax(board: &mut Board, evaluator: &dyn Evaluator, depth: u8) -> (Option<Move>, i32) {
        if depth == 0 {
            return (None, evaluator.evaluate_for_white(&board));
        }

        let mut best_value;
        let mut best_move = None;

        if board.get_active_color() == Color::White {
            best_value = i32::MIN;
            for mv in board.get_legal_moves() {
                board.apply_move(&mv);
                let eval = MinimaxSearch::minimax(board, evaluator, depth - 1).1;
                if eval > best_value {
                    best_value = eval;
                    best_move = Some(mv);
                }
                board.undo_move(&mv);
            }
        } else {
            best_value = i32::MAX;
            for mv in board.get_legal_moves() {
                board.apply_move(&mv);
                let eval = MinimaxSearch::minimax(board, evaluator, depth - 1).1;
                if eval < best_value {
                    best_value = eval;
                    best_move = Some(mv);
                }
                board.undo_move(&mv);
            }
        }
        (best_move, best_value)
    }
}

impl SearchAlgorithm for MinimaxSearch {
    fn find_best_move(board: &Board, evaluator: &dyn Evaluator, depth: u8) -> Option<Move> {
        if depth < 1 {
            panic!("Depth must be at least 1");
        }
        let mut board_clone = board.clone();
        let (best_move, _score) = MinimaxSearch::minimax(&mut board_clone, evaluator, depth);
        return best_move;
    }
}
