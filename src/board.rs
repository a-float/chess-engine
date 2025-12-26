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

#[derive(Default)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

pub struct Board {
    squares: SquareArray,
    pub is_white_turn: bool,
    pub en_passant_square: Option<Square>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
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
}
