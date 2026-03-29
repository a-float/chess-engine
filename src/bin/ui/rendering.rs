use std::time::Instant;

use super::app::App;
use checkmatier::board::{piece, square::Square};
use checkmatier::r#move::get_square_attackers;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::{border, merge::MergeStrategy},
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

pub const WHITE_ACTIVE_COLOR: Color = Color::Rgb(255, 165, 0);
pub const BLACK_ACTIVE_COLOR: Color = Color::Rgb(0, 0, 205);
pub const MUTED_COLOR: Color = Color::Rgb(164, 164, 164);

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

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

            if !white_attackers.is_empty() {
                let attackers_str = white_attackers
                    .iter()
                    .map(|(attacker, square)| format!("{} {}", attacker.to_char(), square))
                    .collect::<Vec<String>>()
                    .join(" ");

                lines.push(Line::from(vec![
                    "Attacked by: ".into(),
                    attackers_str.fg(WHITE_ACTIVE_COLOR).bold(),
                ]));
            }

            if !black_attackers.is_empty() {
                let attackers_str = black_attackers
                    .iter()
                    .map(|(attacker, square)| format!("{} {}", attacker.to_char(), square))
                    .collect::<Vec<String>>()
                    .join(" ");

                lines.push(Line::from(vec![
                    "Attacked by: ".into(),
                    attackers_str.fg(BLACK_ACTIVE_COLOR).bold(),
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
            .evaluate_breakdown(&self.board)
            .iter()
            .map(|(name, score)| format!("{}: {}", name, score))
            .collect::<Vec<String>>()
            .join(" | ");

        let duration_option = match (
            self.ai_last_start_move_time,
            self.ai_last_end_move_time,
            self.ai_searching,
        ) {
            (Some(start), _, true) => Some(Instant::now().duration_since(start)),
            (Some(start), Some(end), false) => Some(end.duration_since(start)),
            _ => None,
        };
        let time_line = if let Some(duration) = duration_option {
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
                if self.ai_searching {
                    " [searching...]".into()
                } else {
                    "".into()
                },
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
