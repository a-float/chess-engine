use crate::Board;
use crate::board::piece::{Color, PieceKind};
use crate::board::square::Square;
use crate::evaluate::Evaluator;

pub struct PositioningEvaluator {
    weight: i32,
}

impl PositioningEvaluator {
    pub fn new(weight: i32) -> Self {
        Self { weight }
    }
}

impl Evaluator for PositioningEvaluator {
    fn name(&self) -> String {
        "positioning".to_string()
    }
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;
        for idx in 0..64 {
            let square = Square::from_index(idx).unwrap();
            if let Some(piece) = board.get_piece(square) {
                let color_aware_rank = if piece.get_color() == Color::White {
                    7 - square.rank as usize
                } else {
                    square.rank as usize
                };
                let piece_value = match piece.get_kind() {
                    PieceKind::Pawn => PAWN_TABLE[color_aware_rank][square.file as usize],
                    PieceKind::Knight => KNIGHT_TABLE[color_aware_rank][square.file as usize],
                    PieceKind::Bishop => BISHOP_TABLE[color_aware_rank][square.file as usize],
                    PieceKind::Rook => ROOK_TABLE[color_aware_rank][square.file as usize],
                    PieceKind::Queen => QUEEN_TABLE[color_aware_rank][square.file as usize],
                    PieceKind::King => KING_TABLE[color_aware_rank][square.file as usize],
                };
                if piece.get_color() == Color::White {
                    score += piece_value;
                } else {
                    score -= piece_value;
                }
            }
        }
        score * board.get_active_color().get_value() * self.weight
    }
}

// Piece-square tables: positive values indicate good squares for White pieces
type PieceSquareTable = [[i32; 8]; 8];

#[rustfmt::skip]
const PAWN_TABLE: PieceSquareTable = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5,  5, 10, 25, 25, 10,  5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [5, 10, 10,-20,-20, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

#[rustfmt::skip]
const KNIGHT_TABLE: PieceSquareTable = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

#[rustfmt::skip]
const BISHOP_TABLE: PieceSquareTable = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

#[rustfmt::skip]
const ROOK_TABLE: PieceSquareTable = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [0,  0,  0,  5,  5,  0,  0,  0],
];

#[rustfmt::skip]
const QUEEN_TABLE: PieceSquareTable = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [-5,  0,  5,  5,  5,  5,  0, -5],
    [0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

#[rustfmt::skip]
const KING_TABLE: PieceSquareTable = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [20, 20,  0,  0,  0,  0, 20, 20],
    [20, 30, 10,  0,  0, 10, 30, 20],
];
