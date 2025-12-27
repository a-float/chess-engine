use super::Board;
use crate::board::square::Square;
use std::fmt::{Display, Formatter};

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        let castling_str = {
            let mut s = String::new();
            if self.castling_rights.white_king_side {
                s.push('K');
            }
            if self.castling_rights.white_queen_side {
                s.push('Q');
            }
            if self.castling_rights.black_king_side {
                s.push('k');
            }
            if self.castling_rights.black_queen_side {
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
                let square = Square { rank, file };
                board_str.push(self.get_piece(square).map_or(' ', |p| p.to_char()));
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
                .map_or("None".to_string(), |sq| format!("{:?}", sq))
        ));
        board_str.push_str(&format!(
            "{} to move {}\n",
            if self.is_white_turn { "White" } else { "Black" },
            if self.is_white_turn { "♟" } else { "♙" }
        ));
        write!(f, "{}", board_str)
    }
}
