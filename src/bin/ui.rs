use board::Board;
use chess_engine::board;
use chess_engine::evaluate::{MaterialEvaluator, PositioningEvaluator, SumEvaluator};
use chess_engine::r#move;
use chess_engine::r#move::get_square_attackers;
use chess_engine::search::{MinimaxSearch, SearchAlgorithm};

use std::{
    cell::Cell,
    io::{self, Error, stdout},
    time::{Duration, Instant},
};

use ratatui::{
    buffer::Buffer,
    crossterm::{
        ExecutableCommand,
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
            MouseEvent, MouseEventKind,
        },
    },
    layout::{Constraint, Layout, Position, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::{border, merge::MergeStrategy},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
    *,
};

use crate::{
    board::{piece, square::Square},
    r#move::Move,
};

const WHITE_ACTIVE_COLOR: Color = Color::Rgb(255, 165, 0);
const BLACK_ACTIVE_COLOR: Color = Color::Rgb(0, 0, 205);
const MUTED_COLOR: Color = Color::Rgb(164, 164, 164);

pub struct App {
    board: Board,
    active_square: Option<Square>,
    possible_moves: Vec<Move>,
    exit: bool,
    board_area: Cell<Rect>,
    move_history: Vec<Move>,
    ai_enabled: bool,
    ai_color: piece::Color,
    ai_depth: u8,
    ai_evaluator: SumEvaluator,
    ai_last_move_time: Option<Duration>,
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
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        stdout().execute(EnableMouseCapture).unwrap();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        stdout().execute(DisableMouseCapture).unwrap();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),
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

    fn undo_move(&mut self) {
        if let Some(last_move) = self.move_history.pop() {
            self.board.undo_move(&last_move);
        }
    }

    fn restart_game(&mut self) {
        while !self.move_history.is_empty() {
            self.undo_move();
        }
    }

    fn toggle_ai(&mut self) {
        self.ai_enabled = !self.ai_enabled;
    }

    fn toggle_ai_color(&mut self) {
        self.ai_color = self.ai_color.opposite();
    }

    fn increase_ai_depth(&mut self) {
        if self.ai_depth < 10 {
            self.ai_depth += 1;
        }
    }

    fn decrease_ai_depth(&mut self) {
        if self.ai_depth > 1 {
            self.ai_depth -= 1;
        }
    }

    fn make_ai_move(&mut self) {
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

    fn check_and_make_ai_move(&mut self) {
        if self.ai_enabled && self.board.get_active_color() == self.ai_color {
            self.make_ai_move();
        }
    }

    fn handle_board_click(&mut self, mouse_event: MouseEvent) -> () {
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
            // || end_move.is_none() && piece.is_none()
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

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
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
        Ok(())
    }
}

impl App {
    fn get_active_style_for_side(color: piece::Color) -> Style {
        match color {
            piece::Color::Black => Style::default().fg(BLACK_ACTIVE_COLOR),
            piece::Color::White => Style::default().fg(WHITE_ACTIVE_COLOR),
        }
    }

    fn get_style_for_square(&self, square: Square) -> Style {
        let dest_move = self.possible_moves.iter().find(|m| m.to == square);
        let piece = self.board.get_piece(square);
        let active_piece = self.active_square.and_then(|s| self.board.get_piece(s));

        match square {
            _ if dest_move.is_some() && dest_move.unwrap().capture.is_some() => {
                Style::default().fg(Color::LightRed)
            }
            _ if dest_move.is_some() => {
                App::get_active_style_for_side(dest_move.unwrap().piece.get_color())
            }
            sq if self.active_square == Some(sq) && active_piece.is_some() => {
                App::get_active_style_for_side(active_piece.unwrap().get_color())
            }
            _ if piece.is_some() && piece.unwrap().get_color() == piece::Color::Black => {
                Style::default().fg(Color::Rgb(196, 196, 196))
            }
            _ if piece.is_some() && piece.unwrap().get_color() == piece::Color::White => {
                Style::default().fg(Color::Rgb(232, 232, 232))
            }
            _ => Style::default(),
        }
    }

    fn get_content_for_square(&self, square: Square) -> String {
        let piece = self.board.get_piece(square);
        let mut char = ' ';
        if piece.is_some() {
            char = piece.unwrap().to_char();
        } else if piece.is_none() && Some(square) == self.active_square {
            char = '?';
        } else if self.possible_moves.iter().any(|m| m.to == square) {
            char = '.';
        }
        format!("{} ", char)
    }

    fn render_board(&self, area: Rect, buf: &mut Buffer) {
        let cols_constraints = (0..9).map(|_| Constraint::Length(6));
        let rows_constraints = (0..9).map(|_| Constraint::Length(3));

        let horizontal = Layout::horizontal(cols_constraints).spacing(Spacing::Overlap(1));
        let vertical = Layout::vertical(rows_constraints).spacing(Spacing::Overlap(1));

        let rows = vertical.split(area);

        let cells: Vec<Vec<Rect>> = rows
            .iter()
            .map(|&row| horizontal.split(row).to_vec())
            .collect();

        for y in 0..8 {
            Paragraph::new(format!("{} ", y + 1))
                .block(
                    Block::new()
                        .padding(Padding::uniform(1))
                        .merge_borders(MergeStrategy::Exact),
                )
                .centered()
                .render(cells[7 - y][0], buf)
        }

        for x in 0..8 {
            Paragraph::new(format!("{} ", (b'A' + x as u8) as char))
                .block(
                    Block::new()
                        .padding(Padding::uniform(1))
                        .merge_borders(MergeStrategy::Exact),
                )
                .centered()
                .render(cells[8][x + 1], buf)
        }

        for y in 0..8 {
            for x in 0..8 {
                let square = Square::from_index(((7 - y) * 8 + x) as u8).unwrap();

                let cell = cells[y as usize][(x + 1) as usize];
                let text = self.get_content_for_square(square);
                let style = self.get_style_for_square(square);
                let block = Block::bordered().merge_borders(MergeStrategy::Exact);

                Paragraph::new(Line::from(text).style(style))
                    .block(block)
                    .centered()
                    .render(cell, buf)
            }
        }
    }

    fn render_game_state(&self, area: Rect, buf: &mut Buffer) {
        let game_state = self.board.get_game_state();

        let check_state_line = if self.board.is_checkmate() {
            let winner = self.board.get_active_color().opposite();
            Line::from(format!("CHECKMATE: {} has won!", winner))
        } else {
            Line::from(vec![
                "Is in check: ".into(),
                format!("{}", self.board.is_in_check()).bold(),
            ])
        };

        let mut lines = vec![
            Line::from(vec![
                format!("{}", self.board.get_active_color()).bold(),
                " to move".into(),
            ]),
            Line::from(""),
            check_state_line,
            Line::from(""),
            Line::from(vec![
                "Move: ".into(),
                format!("{}", self.board.fullmove_number).bold(),
            ]),
            Line::from(vec![
                "Halfmove clock: ".into(),
                format!("{}", game_state.halfmove_clock).bold(),
            ]),
            Line::from(""),
            Line::from(vec![
                "Castling: ".into(),
                self.board.get_casting_str().bold(),
            ]),
            Line::from(""),
            Line::from(vec!["Fen: ".into(), self.board.to_fen().bold()]),
        ];

        if let Some(ep_square) = game_state.en_passant_square {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                "En passant: ".into(),
                format!("{}", ep_square).bold(),
            ]));
        }

        if let Some(active_sq) = self.active_square {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                "Selected: ".into(),
                format!("{}", active_sq).yellow().bold(),
            ]));
            lines.push(Line::from(vec![
                "Possible moves: ".into(),
                format!("{}", self.possible_moves.len()).cyan().bold(),
            ]));
        }

        if let Some(active_sq) = self.active_square {
            let mut mut_board = self.board.clone();
            let black_attackers =
                get_square_attackers(&mut mut_board, active_sq, piece::Color::White);
            let white_attackers =
                get_square_attackers(&mut mut_board, active_sq, piece::Color::Black);

            for (attacker, square) in white_attackers {
                lines.push(Line::from(vec![
                    "Attacked by: ".into(),
                    format!("{}  from {}", attacker.to_char(), square)
                        .fg(WHITE_ACTIVE_COLOR)
                        .bold(),
                ]));
            }

            for (attacker, square) in black_attackers {
                lines.push(Line::from(vec![
                    "Attacked by: ".into(),
                    format!("{}  from {}", attacker.to_char(), square)
                        .fg(BLACK_ACTIVE_COLOR)
                        .bold(),
                ]));
            }
        }

        Paragraph::new(lines)
            .block(
                Block::bordered()
                    .title("Game State")
                    .padding(Padding::uniform(1)),
            )
            .render(area, buf);
    }

    fn render_ai_info(&self, area: Rect, buf: &mut Buffer) {
        let eval_line = self
            .ai_evaluator
            .get_evaluators()
            .iter()
            .map(|e| format!("{}: {}", e.name(), e.evaluate_for_white(&self.board)))
            .collect::<Vec<String>>()
            .join(" | ");

        let time_line = if let Some(duration) = self.ai_last_move_time {
            format!("Last move: {:.2}s", duration.as_secs_f64())
        } else {
            "Last move: N/A".to_string()
        };

        let lines = vec![
            Line::from(vec![
                "AI: ".into(),
                if self.ai_enabled {
                    "ON".green().bold()
                } else {
                    "OFF".red().bold()
                },
                " (".into(),
                format!("{}", self.ai_color).cyan().into(),
                ", depth: ".into(),
                format!("{}", self.ai_depth).yellow().bold(),
                ")".into(),
            ]),
            Line::from(eval_line).fg(Color::Cyan),
            Line::from(time_line).fg(Color::Yellow),
            Line::from("a: toggle | c: color | +/-: depth | m: move").fg(MUTED_COLOR),
        ];

        Paragraph::new(lines)
            .block(
                Block::bordered()
                    .title("AI Settings")
                    .padding(Padding::uniform(1)),
            )
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Length(48), Constraint::Fill(1)]);
        let [left_area, right_area] = horizontal.areas(area);
        let vertical_left = Layout::vertical([Constraint::Length(48), Constraint::Length(2)]);
        let [top_area, bottom_area] = vertical_left.areas(left_area);
        let vertical_right = Layout::vertical([Constraint::Fill(1), Constraint::Length(8)]);
        let [game_state_area, ai_info_area] = vertical_right.areas(right_area);

        let title = Line::from(" Chess ".bold());

        let board_wrapper = Block::new().padding(Padding::top(1));
        let board_area = board_wrapper.inner(left_area);
        self.board_area.set(board_area);

        board_wrapper.render(top_area, buf);

        self.render_board(board_area, buf);

        self.render_game_state(game_state_area, buf);
        self.render_ai_info(ai_info_area, buf);

        Paragraph::new(Line::from(" q: quit | u: undo | r: restart").fg(MUTED_COLOR))
            .centered()
            .render(bottom_area, buf);

        Block::bordered()
            .title_top(title.centered())
            .border_set(border::THICK)
            .render(area, buf);
    }
}

fn main() -> Result<(), Error> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
