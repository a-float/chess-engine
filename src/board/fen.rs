use crate::board::{CastlingRights, GameState, piece::Piece};

use super::{Board, SquareArray};

const FEN_CHARS: [(char, Piece); 12] = [
    ('P', Piece::BLACK_PAWN),
    ('R', Piece::BLACK_ROOK),
    ('N', Piece::BLACK_KNIGHT),
    ('B', Piece::BLACK_BISHOP),
    ('Q', Piece::BLACK_QUEEN),
    ('K', Piece::BLACK_KING),
    ('p', Piece::WHITE_PAWN),
    ('r', Piece::WHITE_ROOK),
    ('n', Piece::WHITE_KNIGHT),
    ('b', Piece::WHITE_BISHOP),
    ('q', Piece::WHITE_QUEEN),
    ('k', Piece::WHITE_KING),
];

fn read_pieces(piece_placement: &str) -> SquareArray {
    let mut pieces = [None; 64];
    let mut i = 0;
    for val in piece_placement.chars() {
        if let Some(digit) = val.to_digit(10) {
            i += digit as usize;
            continue;
        }

        if let Some((_, square)) = FEN_CHARS.iter().find(|(c, _)| *c == val) {
            pieces[i] = Some(*square);
            i += 1;
        }
    }
    pieces
}

fn read_castling_rights(castling_str: &str) -> CastlingRights {
    let mut rights = CastlingRights::default();
    for ch in castling_str.chars() {
        match ch {
            'K' => rights.white_king_side = true,
            'Q' => rights.white_queen_side = true,
            'k' => rights.black_king_side = true,
            'q' => rights.black_queen_side = true,
            _ => panic!("Invalid castling rights character: {}", ch),
        }
    }
    rights
}

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let groups: Vec<&str> = fen.split(' ').collect();
        let piece_placement = groups[0];
        let active_color = groups[1];
        let castling_rights = groups[2];

        return Board {
            squares: read_pieces(piece_placement),
            is_white_turn: active_color == "w",
            fullmove_number: 1,
            state_history: vec![GameState {
                castling_rights: read_castling_rights(castling_rights),
                en_passant_square: None,
                halfmove_clock: 0,
            }],
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pieces() {
        let fen_pieces = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board_array = read_pieces(fen_pieces);

        assert_eq!(board_array[0], Some(Piece::WHITE_ROOK));
        assert_eq!(board_array[1], Some(Piece::WHITE_KNIGHT));
        assert_eq!(board_array[2], Some(Piece::WHITE_BISHOP));
        assert_eq!(board_array[3], Some(Piece::WHITE_QUEEN));
        assert_eq!(board_array[4], Some(Piece::WHITE_KING));
        assert_eq!(board_array[5], Some(Piece::WHITE_BISHOP));
        assert_eq!(board_array[6], Some(Piece::WHITE_KNIGHT));
        assert_eq!(board_array[7], Some(Piece::WHITE_ROOK));

        for i in 8..16 {
            assert_eq!(board_array[i], Some(Piece::WHITE_PAWN));
        }
        for i in 16..48 {
            assert_eq!(board_array[i], None);
        }
        for i in 48..56 {
            assert_eq!(board_array[i], Some(Piece::BLACK_PAWN));
        }

        assert_eq!(board_array[56], Some(Piece::BLACK_ROOK));
        assert_eq!(board_array[57], Some(Piece::BLACK_KNIGHT));
        assert_eq!(board_array[58], Some(Piece::BLACK_BISHOP));
        assert_eq!(board_array[59], Some(Piece::BLACK_QUEEN));
        assert_eq!(board_array[60], Some(Piece::BLACK_KING));
        assert_eq!(board_array[61], Some(Piece::BLACK_BISHOP));
        assert_eq!(board_array[62], Some(Piece::BLACK_KNIGHT));
        assert_eq!(board_array[63], Some(Piece::BLACK_ROOK));
    }
}
