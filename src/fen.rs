use super::board::{Board, SquareArray};
use super::constants::*;

const FEN_CHARS: [(char, u8); 12] = [
    ('P', PieceConst::BLACK_PAWN),
    ('R', PieceConst::BLACK_ROOK),
    ('N', PieceConst::BLACK_KNIGHT),
    ('B', PieceConst::BLACK_BISHOP),
    ('Q', PieceConst::BLACK_QUEEN),
    ('K', PieceConst::BLACK_KING),
    ('p', PieceConst::WHITE_PAWN),
    ('r', PieceConst::WHITE_ROOK),
    ('n', PieceConst::WHITE_KNIGHT),
    ('b', PieceConst::WHITE_BISHOP),
    ('q', PieceConst::WHITE_QUEEN),
    ('k', PieceConst::WHITE_KING),
];

fn read_pieces(piece_placement: &str) -> SquareArray {
    let mut pieces = [PieceConst::EMPTY; 64];
    let mut i = 0;
    for val in piece_placement.chars() {
        if let Some(digit) = val.to_digit(10) {
            i += digit as usize;
            continue;
        }

        if let Some((_, square)) = FEN_CHARS.iter().find(|(c, _)| *c == val) {
            pieces[i] = *square;
            i += 1;
        }
    }
    pieces
}

fn read_castling_rights(castling_str: &str) -> CastlingRights {
    let mut rights = 0;
    for ch in castling_str.chars() {
        match ch {
            'K' => rights |= CastlingRightsConst::WHITE_KING_SIDE,
            'Q' => rights |= CastlingRightsConst::WHITE_QUEEN_SIDE,
            'k' => rights |= CastlingRightsConst::BLACK_KING_SIDE,
            'q' => rights |= CastlingRightsConst::BLACK_QUEEN_SIDE,
            _ => {}
        }
    }
    rights
}

pub fn from_fen(fen: &str) -> Board {
    let groups: Vec<&str> = fen.split(' ').collect();
    let piece_placement = groups[0];
    let active_color = groups[1];
    let castling_rights = groups[2];

    return Board {
        squares: read_pieces(piece_placement),
        is_white_turn: active_color == "w",
        castling_rights: read_castling_rights(castling_rights),
        en_passant_square: None,
        halfmove_clock: 0,
        fullmove_number: 1,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pieces() {
        let fen_pieces = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board_array = read_pieces(fen_pieces);

        assert_eq!(board_array[0], PieceConst::WHITE_ROOK);
        assert_eq!(board_array[1], PieceConst::WHITE_KNIGHT);
        assert_eq!(board_array[2], PieceConst::WHITE_BISHOP);
        assert_eq!(board_array[3], PieceConst::WHITE_QUEEN);
        assert_eq!(board_array[4], PieceConst::WHITE_KING);
        assert_eq!(board_array[5], PieceConst::WHITE_BISHOP);
        assert_eq!(board_array[6], PieceConst::WHITE_KNIGHT);
        assert_eq!(board_array[7], PieceConst::WHITE_ROOK);

        for i in 8..16 {
            assert_eq!(board_array[i], PieceConst::WHITE_PAWN);
        }
        for i in 16..48 {
            assert_eq!(board_array[i], PieceConst::EMPTY);
        }
        for i in 48..56 {
            assert_eq!(board_array[i], PieceConst::BLACK_PAWN);
        }

        assert_eq!(board_array[56], PieceConst::BLACK_ROOK);
        assert_eq!(board_array[57], PieceConst::BLACK_KNIGHT);
        assert_eq!(board_array[58], PieceConst::BLACK_BISHOP);
        assert_eq!(board_array[59], PieceConst::BLACK_QUEEN);
        assert_eq!(board_array[60], PieceConst::BLACK_KING);
        assert_eq!(board_array[61], PieceConst::BLACK_BISHOP);
        assert_eq!(board_array[62], PieceConst::BLACK_KNIGHT);
        assert_eq!(board_array[63], PieceConst::BLACK_ROOK);
    }
}
