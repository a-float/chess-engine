mod display;
mod fen;
pub mod piece;
pub mod square;

use crate::{
    board::{
        piece::{Color, Piece, PieceKind},
        square::Square,
    },
    r#move::{Move, get_moves_from_square, get_square_attackers},
};

pub type SquareArray = [Option<Piece>; 64];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

    pub fn is_square_attacked(&self, square: Square, by_color: Color) -> bool {
        let attackers = get_square_attackers(self, square, by_color);
        !attackers.is_empty()
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

    // fn get_all_moves(&self) -> Vec<Move> {
    //     self.get_moves_for_color(self.get_active_color())
    // }

    pub fn get_legal_moves_for_color(&self, color: Color) -> Vec<Move> {
        let all = self.get_moves_for_color(color);
        let mut legal_moves = Vec::new();
        let mut mock_board = self.clone();
        for m in all {
            mock_board.apply_move(&m);
            // apply_move toggles active color, we toggle it again checking for opponent check
            if !mock_board.is_opponent_in_check() {
                legal_moves.push(m);
            }
            mock_board.undo_move(&m);
        }
        legal_moves
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.get_legal_moves_for_color(self.get_active_color())
    }

    fn get_moves_for_color(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for square_idx in 0..64 {
            let square = Square::from_index(square_idx).unwrap();
            if let Some(piece) = self.get_piece(square)
                && piece.get_color() == color
            {
                moves.extend(get_moves_from_square(&self, square));
            }
        }
        moves
    }

    fn get_checking_pieces(&self, color: Color) -> Vec<(Piece, Square)> {
        let target_piece = Piece::new(color, PieceKind::King);
        let king_square_option = (0..64).find_map(|idx| {
            let square = Square::from_index(idx).unwrap();
            if self.get_piece(square) == Some(target_piece) {
                Some(square)
            } else {
                None
            }
        });

        if let Some(king_square) = king_square_option {
            get_square_attackers(self, king_square, color)
        } else {
            Vec::new() // Can't happen on a valid board
        }
    }

    pub fn is_color_in_check(&self, color: Color) -> bool {
        !self.get_checking_pieces(color).is_empty()
    }

    pub fn is_in_check(&self) -> bool {
        self.is_color_in_check(self.get_active_color())
    }

    pub fn is_opponent_in_check(&self) -> bool {
        self.is_color_in_check(self.get_active_color().opposite())
    }

    pub fn is_checkmate(&self) -> bool {
        self.get_legal_moves().is_empty() && self.is_in_check()
    }

    pub fn is_draw(&self) -> bool {
        self.get_legal_moves().is_empty() && !self.is_in_check()
    }

    fn toggle_active_color(&mut self) {
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn get_move_from_algebraic_notation(&self, notation: &str) -> Option<Move> {
        self.get_legal_moves()
            .iter()
            .find(|m| m.to_long_algebraic_notation() == notation)
            .copied()
    }

    fn update_castling_rights(rights: &mut CastlingRights, m: &Move) {
        if m.piece == Piece::BLACK_KING {
            rights.black_king_side = false;
            rights.black_queen_side = false;
        }
        if m.piece == Piece::WHITE_KING {
            rights.white_king_side = false;
            rights.white_queen_side = false;
        }
        if m.piece == Piece::BLACK_ROOK {
            match m.from {
                s if s == Square { file: 0, rank: 7 } => rights.black_queen_side = false,
                s if s == Square { file: 7, rank: 7 } => rights.black_king_side = false,
                _ => {}
            }
        }
        if m.piece == Piece::WHITE_ROOK {
            match m.from {
                s if s == Square { file: 0, rank: 0 } => rights.white_queen_side = false,
                s if s == Square { file: 7, rank: 0 } => rights.white_king_side = false,
                _ => {}
            }
        }
    }

    pub fn apply_move(&mut self, m: &Move) {
        if m.piece.get_color() == Color::Black {
            self.fullmove_number += 1;
        }
        self.toggle_active_color();
        self.set_piece(m.to, m.promotion.or(Some(m.piece)));
        self.set_piece(m.from, None);

        let prev_state = self.get_game_state();
        let mut new_game_state = GameState {
            en_passant_square: m.en_passant_square,
            castling_rights: prev_state.castling_rights,
            halfmove_clock: if m.capture.is_some() {
                0
            } else {
                prev_state.halfmove_clock + 1
            },
        };

        Self::update_castling_rights(&mut new_game_state.castling_rights, m);

        if let Some(sq) = m.en_passant_square
            && m.capture.is_some()
        {
            self.set_piece(sq, None);
            new_game_state.en_passant_square = None;
        }

        if let Some((rook_from, rook_to)) = m.castling_rook_from_to {
            self.set_piece(rook_to, self.get_piece(rook_from));
            self.set_piece(rook_from, None);
        }

        self.state_history.push(new_game_state);
    }

    pub fn undo_move(&mut self, m: &Move) {
        if m.piece.get_color() == Color::Black {
            self.fullmove_number -= 1;
        }
        self.toggle_active_color();
        self.set_piece(m.from, m.promotion.or(Some(m.piece)));
        if let Some(sq) = m.en_passant_square
            && m.capture.is_some()
        {
            self.set_piece(m.to, None);
            self.set_piece(sq, m.capture);
        } else {
            self.set_piece(m.to, m.capture);
        }
        if let Some((rook_from, rook_to)) = m.castling_rook_from_to {
            self.set_piece(rook_from, self.get_piece(rook_to));
            self.set_piece(rook_to, None);
        }

        self.state_history.pop();
    }
}
