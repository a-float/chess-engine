#![allow(dead_code)]
pub type Piece = u8;
pub type SquareIndex = u8;
pub type Color = u8;
pub type CastlingRights = u8;

#[derive(PartialEq, Clone, Copy)]
pub struct PieceType;

impl PieceType {
    pub const EMPTY: Piece = 0;
    pub const PAWN: Piece = 1 << 0;
    pub const ROOK: Piece = 1 << 1;
    pub const KNIGHT: Piece = 1 << 2;
    pub const BISHOP: Piece = 1 << 3;
    pub const QUEEN: Piece = 1 << 4;
    pub const KING: Piece = 1 << 5;
}

pub struct PieceColor;

impl PieceColor {
    pub const WHITE: Piece = 1 << 6;
    pub const BLACK: Piece = 0 << 6;
}

pub struct CastlingRightsConst;

impl CastlingRightsConst {
    pub const WHITE_KING_SIDE: CastlingRights = 1 << 0;
    pub const WHITE_QUEEN_SIDE: CastlingRights = 1 << 1;
    pub const BLACK_KING_SIDE: CastlingRights = 1 << 2;
    pub const BLACK_QUEEN_SIDE: CastlingRights = 1 << 3;
}

pub struct PieceConst;

impl PieceConst {
    pub const EMPTY: Piece = 0;
    pub const WHITE_PAWN: Piece = PieceColor::WHITE | PieceType::PAWN;
    pub const WHITE_ROOK: Piece = PieceColor::WHITE | PieceType::ROOK;
    pub const WHITE_KNIGHT: Piece = PieceColor::WHITE | PieceType::KNIGHT;
    pub const WHITE_BISHOP: Piece = PieceColor::WHITE | PieceType::BISHOP;
    pub const WHITE_QUEEN: Piece = PieceColor::WHITE | PieceType::QUEEN;
    pub const WHITE_KING: Piece = PieceColor::WHITE | PieceType::KING;
    pub const BLACK_PAWN: Piece = PieceColor::BLACK | PieceType::PAWN;
    pub const BLACK_ROOK: Piece = PieceColor::BLACK | PieceType::ROOK;
    pub const BLACK_KNIGHT: Piece = PieceColor::BLACK | PieceType::KNIGHT;
    pub const BLACK_BISHOP: Piece = PieceColor::BLACK | PieceType::BISHOP;
    pub const BLACK_QUEEN: Piece = PieceColor::BLACK | PieceType::QUEEN;
    pub const BLACK_KING: Piece = PieceColor::BLACK | PieceType::KING;
}

pub struct Square;

impl Square {
    pub fn get_square_name(square_idx: SquareIndex) -> String {
        let file = (square_idx - 1) % 8;
        let rank = (square_idx - 1) / 8;
        let file_char = (b'a' + file) as char;
        let rank_char = (b'1' + rank) as char;
        format!("{}{}", file_char, rank_char)
    }

    pub const A1: SquareIndex = 0;
    pub const A2: SquareIndex = 1;
    pub const A3: SquareIndex = 2;
    pub const A4: SquareIndex = 3;
    pub const A5: SquareIndex = 4;
    pub const A6: SquareIndex = 5;
    pub const A7: SquareIndex = 6;
    pub const A8: SquareIndex = 7;
    pub const B1: SquareIndex = 8;
    pub const B2: SquareIndex = 9;
    pub const B3: SquareIndex = 10;
    pub const B4: SquareIndex = 11;
    pub const B5: SquareIndex = 12;
    pub const B6: SquareIndex = 13;
    pub const B7: SquareIndex = 14;
    pub const B8: SquareIndex = 15;
    pub const C1: SquareIndex = 16;
    pub const C2: SquareIndex = 17;
    pub const C3: SquareIndex = 18;
    pub const C4: SquareIndex = 19;
    pub const C5: SquareIndex = 20;
    pub const C6: SquareIndex = 21;
    pub const C7: SquareIndex = 22;
    pub const C8: SquareIndex = 23;
    pub const D1: SquareIndex = 24;
    pub const D2: SquareIndex = 25;
    pub const D3: SquareIndex = 26;
    pub const D4: SquareIndex = 27;
    pub const D5: SquareIndex = 28;
    pub const D6: SquareIndex = 29;
    pub const D7: SquareIndex = 30;
    pub const D8: SquareIndex = 31;
    pub const E1: SquareIndex = 32;
    pub const E2: SquareIndex = 33;
    pub const E3: SquareIndex = 34;
    pub const E4: SquareIndex = 35;
    pub const E5: SquareIndex = 36;
    pub const E6: SquareIndex = 37;
    pub const E7: SquareIndex = 38;
    pub const E8: SquareIndex = 39;
    pub const F1: SquareIndex = 40;
    pub const F2: SquareIndex = 41;
    pub const F3: SquareIndex = 42;
    pub const F4: SquareIndex = 43;
    pub const F5: SquareIndex = 44;
    pub const F6: SquareIndex = 45;
    pub const F7: SquareIndex = 46;
    pub const F8: SquareIndex = 47;
    pub const G1: SquareIndex = 48;
    pub const G2: SquareIndex = 49;
    pub const G3: SquareIndex = 50;
    pub const G4: SquareIndex = 51;
    pub const G5: SquareIndex = 52;
    pub const G6: SquareIndex = 53;
    pub const G7: SquareIndex = 54;
    pub const G8: SquareIndex = 55;
    pub const H1: SquareIndex = 56;
    pub const H2: SquareIndex = 57;
    pub const H3: SquareIndex = 58;
    pub const H4: SquareIndex = 59;
    pub const H5: SquareIndex = 60;
    pub const H6: SquareIndex = 61;
    pub const H7: SquareIndex = 62;
    pub const H8: SquareIndex = 63;
}
