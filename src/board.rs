use std::fmt::{Display, Formatter};

use super::constants::*;

fn get_piece_color(piece: Piece) -> Color {
    piece & (1 << 6)
}

fn get_piece_type(piece: Piece) -> Piece {
    piece & 0b0011_1111
}

pub type SquareArray = [Piece; 64];

pub struct Board {
    pub squares: SquareArray,
    pub is_white_turn: bool,
    pub en_passant_square: Option<SquareIndex>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
}

fn piece_char(piece: Piece) -> char {
    match piece {
        PieceConst::BLACK_BISHOP => '♗',
        PieceConst::BLACK_KING => '♔',
        PieceConst::BLACK_KNIGHT => '♘',
        PieceConst::BLACK_PAWN => '♙',
        PieceConst::BLACK_QUEEN => '♕',
        PieceConst::BLACK_ROOK => '♖',
        PieceConst::WHITE_BISHOP => '♝',
        PieceConst::WHITE_KING => '♚',
        PieceConst::WHITE_KNIGHT => '♞',
        PieceConst::WHITE_PAWN => '♟',
        PieceConst::WHITE_QUEEN => '♛',
        PieceConst::WHITE_ROOK => '♜',
        _ => ' ',
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        let castling_str = {
            let mut s = String::new();
            if self.castling_rights & CastlingRightsConst::WHITE_KING_SIDE != 0 {
                s.push('K');
            }
            if self.castling_rights & CastlingRightsConst::WHITE_QUEEN_SIDE != 0 {
                s.push('Q');
            }
            if self.castling_rights & CastlingRightsConst::BLACK_KING_SIDE != 0 {
                s.push('k');
            }
            if self.castling_rights & CastlingRightsConst::BLACK_QUEEN_SIDE != 0 {
                s.push('q');
            }
            if s.is_empty() {
                s.push('-');
            }
            s
        };
        board_str.push_str("  a b c d e f g h \n");
        for rank in (0..8).rev() {
            board_str.push_str(&format!("{} ", rank + 1));
            for file in 0..8 {
                let piece = self.squares[rank * 8 + file];

                board_str.push(piece_char(piece));
                board_str.push(' ');
            }
            board_str.push('\n');
        }
        board_str.push_str("\n");
        board_str.push_str(&format!(
            "Move: {} | Capture clock: {} | Castling rights: {} | En passant: {}\n",
            self.fullmove_number,
            self.halfmove_clock,
            castling_str,
            self.en_passant_square
                .map_or("None".to_string(), |s| Square::get_square_name(s))
        ));
        board_str.push_str(&format!(
            "{} to move {}\n",
            if self.is_white_turn { "White" } else { "Black" },
            if self.is_white_turn { "♟" } else { "♙" }
        ));
        write!(f, "{}", board_str)
    }
}
