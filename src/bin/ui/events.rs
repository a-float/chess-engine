use super::app::App;
use checkmatier::board::square::Square;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};
use ratatui::layout::Position;
use std::io;
use std::time::Duration;

impl App {
    pub fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                Event::Mouse(mouse_event)
                    if mouse_event.kind == MouseEventKind::Up(event::MouseButton::Left) =>
                {
                    if self
                        .board_area
                        .get()
                        .contains(Position::new(mouse_event.column, mouse_event.row))
                    {
                        self.handle_board_click(mouse_event);
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
            KeyCode::Char('u') => self.undo_move(),
            KeyCode::Char('r') => self.restart_game(),
            KeyCode::Char('a') => self.toggle_ai(),
            KeyCode::Char('c') => self.toggle_ai_color(),
            KeyCode::Char('+') | KeyCode::Char('=') => self.increase_ai_depth(),
            KeyCode::Char('-') => self.decrease_ai_depth(),
            KeyCode::Char('m') => self.make_ai_move(),
            _ => {}
        }
    }

    fn handle_board_click(&mut self, mouse_event: MouseEvent) {
        let board_area = self.board_area.get();
        let square_width = 5;
        let square_height = 2;
        let new_square = Square::new(
            ((mouse_event.column - board_area.x) / square_width) as i8 - 1,
            7 - ((mouse_event.row - board_area.y) / square_height) as i8,
        );

        if new_square.is_none() {
            self.active_square = None;
            self.possible_moves.clear();
            return;
        }

        let piece = self.board.get_piece(new_square.unwrap());

        let end_move = self
            .possible_moves
            .iter()
            .find(|m| m.to == new_square.unwrap());

        self.active_square = if new_square == self.active_square
            || piece.is_some_and(|p| p.get_color() != self.board.get_active_color())
        {
            None
        } else {
            new_square
        };

        if let Some(m) = end_move {
            self.board.apply_move(m);
            self.active_square = None;
            self.move_history.push(*m);
            self.check_and_make_ai_move();
        }

        self.possible_moves = self.active_square.map_or(Vec::new(), |square| {
            self.board
                .get_legal_moves()
                .iter()
                .filter(|m| m.from == square)
                .copied()
                .collect()
        });
    }
}
