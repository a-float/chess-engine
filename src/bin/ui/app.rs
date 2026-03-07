use checkmatier::board::{Board, piece, square::Square};
use checkmatier::evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator};
use checkmatier::r#move::Move;
use std::cell::Cell;
use std::time::Duration;
use ratatui::layout::Rect;

pub struct App {
    pub board: Board,
    pub active_square: Option<Square>,
    pub possible_moves: Vec<Move>,
    pub exit: bool,
    pub board_area: Cell<Rect>,
    pub move_history: Vec<Move>,
    pub ai_enabled: bool,
    pub ai_color: piece::Color,
    pub ai_depth: u8,
    pub ai_evaluator: SumEvaluator,
    pub ai_last_move_time: Option<Duration>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            active_square: None,
            possible_moves: Vec::new(),
            exit: false,
            board_area: Cell::new(Rect::default()),
            move_history: Vec::new(),
            ai_enabled: false,
            ai_color: piece::Color::Black,
            ai_depth: 3,
            ai_evaluator: SumEvaluator::new(vec![
                Box::new(MaterialEvaluator::new(10)),
                Box::new(PositioningEvaluator::new(1)),
            ]),
            ai_last_move_time: None,
        }
    }
}

impl App {
    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn undo_move(&mut self) {
        if let Some(last_move) = self.move_history.pop() {
            self.board.undo_move(&last_move);
        }
    }

    pub fn restart_game(&mut self) {
        while !self.move_history.is_empty() {
            self.undo_move();
        }
    }
}
