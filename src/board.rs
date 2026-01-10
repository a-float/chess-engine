mod display;
mod fen;
pub mod piece;
pub mod square;

use crate::{
    board::{
        piece::{Color, Piece},
        square::Square,
    },
    r#move::{Move, get_moves_for_piece},
};

pub type SquareArray = [Option<Piece>; 64];

#[derive(Debug, Default, Clone, Copy)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub en_passant_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u16,
}

#[derive(Debug, Clone)]
pub struct Board {
    squares: SquareArray,
    is_white_turn: bool,
    pub fullmove_number: u16,
    state_history: Vec<GameState>,
}

impl Default for Board {
    fn default() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Board {
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.to_index()]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.to_index()] = piece
    }

    pub fn is_square_empty(&self, square: Square) -> bool {
        self.get_piece(square).is_none()
    }

    pub fn get_active_color(&self) -> Color {
        if self.is_white_turn {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_game_state(&self) -> &GameState {
        return self.state_history.last().unwrap();
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.get_moves_for_color(self.get_active_color())
    }

    pub fn get_moves_for_color(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for square_idx in 0..64 {
            let square = Square::from_index(square_idx).unwrap();
            if let Some(piece) = self.get_piece(square)
                && piece.get_color() == color
            {
                moves.extend(get_moves_for_piece(&self, square));
            }
        }
        moves
    }

    fn toggle_active_color(&mut self) {
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn apply_move(&mut self, m: &Move) {
        if m.piece.get_color() == Color::Black {
            self.fullmove_number += 1;
        }
        self.toggle_active_color();
        self.set_piece(m.to, m.promotion.or(Some(m.piece)));
        self.set_piece(m.from, None);

        let prev_state = self.get_game_state();
        self.state_history.push(GameState {
            en_passant_square: m.en_passant_square,
            castling_rights: prev_state.castling_rights,
            halfmove_clock: if m.capture.is_some() {
                0
            } else {
                prev_state.halfmove_clock + 1
            },
        });
    }

    pub fn undo_move(&mut self, m: &Move) {
        if m.piece.get_color() == Color::Black {
            self.fullmove_number -= 1;
        }
        self.toggle_active_color();
        self.set_piece(m.from, m.promotion.or(Some(m.piece)));
        self.set_piece(m.to, m.capture);
        self.state_history.pop();
    }

    pub fn get_legal_move_from_string(&self, s: &str) -> Option<Move> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let from = Square::from_string(parts[0])?;
        let to = Square::from_string(parts[1])?;

        let mut all_moves = self.get_moves_for_color(Color::White);
        all_moves.extend(self.get_moves_for_color(Color::Black));

        all_moves
            .iter()
            .find(|m| m.from == from && m.to == to)
            .copied()
    }
}
