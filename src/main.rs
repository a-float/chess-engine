mod board;
mod r#move;
mod search;
use board::Board;

use std::io::{self, Error, stdout};

use ratatui::{
    buffer::Buffer,
    crossterm::{
        ExecutableCommand,
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
            MouseEventKind,
        },
    },
    layout::{Constraint, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
    *,
};

use crate::{
    board::{piece, square::Square},
    r#move::{Move, get_moves_for_piece},
};

#[derive(Debug, Default)]
pub struct App {
    board: Board,
    active_square: Option<Square>,
    possible_moves: Vec<Move>,
    exit: bool,
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
            _ => {}
        }
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
                let new_square = Square::new(
                    mouse_event.column as i8 / 5 - 1,
                    7 - (mouse_event.row / 2) as i8,
                );

                if new_square.is_none() {
                    self.active_square = None;
                    self.possible_moves.clear();
                    return Ok(());
                }

                let piece = self.board.get_piece(new_square.unwrap());
                let end_move = self
                    .possible_moves
                    .iter()
                    .find(|m| m.to == new_square.unwrap());

                self.active_square =
                    if new_square == self.active_square || end_move.is_none() && piece.is_none() {
                        None
                    } else {
                        new_square
                    };

                if end_move.is_some() {
                    self.board.apply_move(end_move.unwrap());
                    self.active_square = None;
                }

                self.possible_moves = self.active_square.map_or(Vec::new(), |square| {
                    get_moves_for_piece(&self.board, square)
                });
            }
            _ => {}
        };
        Ok(())
    }
}

impl App {
    fn get_active_style_for_side(color: piece::Color) -> Style {
        match color {
            piece::Color::Black => Style::default().fg(Color::Rgb(0, 0, 205)),
            piece::Color::White => Style::default().fg(Color::Rgb(255, 165, 0)),
        }
    }

    fn get_style_for_square(&self, square: Square) -> Style {
        let dest_move = self.possible_moves.iter().find(|m| m.to == square);
        let piece = self.active_square.and_then(|s| self.board.get_piece(s));

        match square {
            _ if dest_move.is_some() && dest_move.unwrap().capture.is_some() => {
                Style::default().fg(Color::LightRed)
            }
            _ if dest_move.is_some() => {
                App::get_active_style_for_side(dest_move.unwrap().piece.get_color())
            }
            sq if self.active_square == Some(sq) && piece.is_some() => {
                App::get_active_style_for_side(piece.unwrap().get_color())
            }
            _ => Style::default(),
        }
    }

    fn get_content_for_square(&self, square: Square) -> String {
        let piece = self.board.get_piece(square);
        let mut char = ' ';
        if piece.is_some() {
            char = piece.unwrap().to_char();
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
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
        let [left_area, right_area] = horizontal.areas(area);
        // let vertical = Layout::vertical([Constraint::Length(20), Constraint::Fill(1)]);
        // let [top_area, bottom_area] = vertical.areas(left_area);

        let title = Line::from(" Chess ".bold());
        // let block = Block::bordered()
        //     .title(title.centered())
        //     .border_set(border::THICK);

        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.counter.to_string().yellow(),
        // ])]);

        // Paragraph::new("This is text")
        //     .centered()
        //     .render(right_area, buf);

        self.render_board(left_area, buf);

        // Block::bordered()
        //     .title_top(title.centered())
        //     .border_set(border::THICK)
        //     .padding(Padding::symmetric(16, 24))
        //     .render(right_area, buf);
    }
}

fn main() -> Result<(), Error> {
    // color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
