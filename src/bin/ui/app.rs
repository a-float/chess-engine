use checkmatier::board::{Board, piece, square::Square};
use checkmatier::evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator};
use checkmatier::r#move::Move;
use ratatui::layout::Rect;
use std::cell::Cell;
use std::sync::{
    Arc,
    mpsc::{self, Receiver, Sender},
};
use std::time::Instant;

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
    pub ai_evaluator: Arc<SumEvaluator>,
    pub ai_last_start_move_time: Option<Instant>,
    pub ai_last_end_move_time: Option<Instant>,
    // Channel where background AI search threads send their found move (or None)
    pub ai_move_rx: Receiver<Option<Move>>,
    pub ai_move_tx: Sender<Option<Move>>,
    // Whether an AI search is currently running
    pub ai_searching: bool,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

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
            ai_evaluator: Arc::new(SumEvaluator::new(vec![
                Box::new(MaterialEvaluator::new(10)),
                Box::new(PositioningEvaluator::new(1)),
            ])),
            ai_last_start_move_time: None,
            ai_last_end_move_time: None,
            ai_move_rx: rx,
            ai_move_tx: tx,
            ai_searching: false,
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
