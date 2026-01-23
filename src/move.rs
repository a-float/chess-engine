use crate::board::Board;
use crate::board::piece::{Color, Piece, PieceKind};
use crate::board::square::Square;
use std::fmt::{Display, Formatter};

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

const WHITE_PAWN_ATTACK_OFFSETS: [(i8, i8); 2] = [(-1, 1), (1, 1)];
const BLACK_PAWN_ATTACK_OFFSETS: [(i8, i8); 2] = [(-1, -1), (1, -1)];
const BISHOP_OFFSETS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
const ROOK_OFFSETS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const KING_OFFSETS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (1, 0),
    (0, -1),
    (0, 1),
];
const QUEEN_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub capture: Option<Piece>,
    pub piece: Piece,
    pub en_passant_square: Option<Square>,
    pub castling_rook_from_to: Option<(Square, Square)>,
}

impl Move {
    fn new(from: Square, to: Square, piece: Piece) -> Self {
        Self {
            from,
            to,
            piece,
            capture: None,
            en_passant_square: None,
            promotion: None,
            castling_rook_from_to: None,
        }
    }

    fn with_capture(&mut self, capture: Piece) -> Self {
        self.capture = Some(capture);
        *self
    }

    fn with_capture_option(&mut self, capture: Option<Piece>) -> Self {
        self.capture = capture;
        *self
    }

    fn with_promotion(&mut self, promotion: Piece) -> Self {
        self.promotion = Some(promotion);
        *self
    }

    fn with_en_passant_square(&mut self, en_passant_square: Square) -> Self {
        self.en_passant_square = Some(en_passant_square);
        *self
    }

    fn with_castling(&mut self, rook_from: Square, rook_to: Square) -> Self {
        self.castling_rook_from_to = Some((rook_from, rook_to));
        *self
    }
}

fn get_moves_in_line(
    board: &Board,
    square: Square,
    piece: Piece,
    directions: Vec<(i8, i8)>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for (file_delta, rank_delta) in directions.iter() {
        let mut target_square_option = square.offset(*file_delta, *rank_delta);
        while let Some(target_square) = target_square_option {
            if let Some(other_piece) = board.get_piece(target_square) {
                if other_piece.get_color() != piece.get_color() {
                    moves.push(Move::new(square, target_square, piece).with_capture(other_piece));
                }
                break;
            } else {
                moves.push(Move::new(square, target_square, piece));
            }
            target_square_option = target_square.offset(*file_delta, *rank_delta);
        }
    }
    moves
}

fn get_moves_at_offsets(
    board: &Board,
    square: Square,
    piece: Piece,
    offsets: Vec<(i8, i8)>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for target_square in offsets
        .iter()
        .map(|(file_delta, rank_delta)| square.offset(*file_delta, *rank_delta))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
    {
        let other_piece_option = board.get_piece(target_square);
        if other_piece_option.is_none_or(|p| p.get_color() != piece.get_color()) {
            moves.push(
                Move::new(square, target_square, piece).with_capture_option(other_piece_option),
            )
        }
    }
    moves
}

fn get_moves_for_pawn(board: &Board, square: Square, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let dir = match piece.get_color() {
        Color::White => 1,
        Color::Black => -1,
    };
    let forward_square_option = square.offset(0, dir);
    if let Some(forward_square) = forward_square_option
        && board.is_square_empty(forward_square)
    {
        moves.push(Move::new(square, forward_square, piece));

        // double move
        if (dir < 0 && square.rank == 6) || (dir > 0 && square.rank == 1) {
            let double_forward_square_option = square.offset(0, dir * 2);
            if double_forward_square_option.is_some_and(|s| board.is_square_empty(s)) {
                moves.push(
                    Move::new(square, double_forward_square_option.unwrap(), piece)
                        .with_en_passant_square(forward_square),
                );
            }
        }
    }
    let attack_offsets = [(-1, dir), (1, dir)];
    for attack_square in attack_offsets
        .iter()
        .map(|(file_delta, rank_delta)| square.offset(*file_delta, *rank_delta))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
    {
        if let Some(other_piece) = board.get_piece(attack_square)
            && other_piece.get_color() != piece.get_color()
        {
            moves.push(Move::new(square, attack_square, piece).with_capture(other_piece));
        }

        // en passant
        // if both capture and en_passant_square are set, the en_passant square points to captured pawn
        if Some(attack_square) == board.get_game_state().en_passant_square {
            moves.push(
                Move::new(square, attack_square, piece)
                    .with_capture(Piece::new(piece.get_color().opposite(), PieceKind::Pawn))
                    .with_en_passant_square(attack_square.offset(0, -dir).unwrap()),
            )
        }
    }

    return moves
        .iter()
        .flat_map(|m| {
            if m.to.rank == 0 || m.to.rank == 7 {
                [
                    PieceKind::Queen,
                    PieceKind::Rook,
                    PieceKind::Bishop,
                    PieceKind::Knight,
                ]
                .iter()
                .map(|&kind| {
                    Move::new(m.from, m.to, piece)
                        .with_capture_option(m.capture)
                        .with_promotion(Piece::new(piece.get_color(), kind))
                })
                .collect::<Vec<_>>()
            } else {
                vec![*m]
            }
        })
        .collect();
}

fn get_castling_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let castling_rights = board.get_game_state().castling_rights;
    let (piece, rank) = match color {
        Color::White => (Piece::WHITE_KING, 0),
        Color::Black => (Piece::BLACK_KING, 7),
    };
    let (can_king_side, can_queen_side) = match color {
        Color::White => (
            castling_rights.white_king_side,
            castling_rights.white_queen_side,
        ),
        Color::Black => (
            castling_rights.black_king_side,
            castling_rights.black_queen_side,
        ),
    };

    if can_king_side
        && [Square { file: 5, rank: 0 }, Square { file: 6, rank: 0 }]
            .iter()
            .all(|&s| board.is_square_empty(s) && !board.is_square_attacked(s, color))
    {
        moves.push(
            Move::new(Square { file: 4, rank }, Square { file: 6, rank }, piece)
                .with_castling(Square { file: 7, rank }, Square { file: 5, rank }),
        );
    }
    if can_queen_side
        && [
            Square { file: 1, rank },
            Square { file: 2, rank },
            Square { file: 3, rank },
        ]
        .iter()
        .all(|&s| board.is_square_empty(s) && !board.is_square_attacked(s, color))
    {
        moves.push(
            Move::new(Square { file: 4, rank }, Square { file: 2, rank }, piece)
                .with_castling(Square { file: 0, rank }, Square { file: 3, rank }),
        );
    }

    moves
}

pub fn get_square_attackers(
    board: &Board,
    square: Square,
    attacked: Color,
) -> Vec<(Piece, Square)> {
    let mut attackers: Vec<Move> = Vec::new();

    let piece = Piece::new(attacked, PieceKind::Pawn); // dummy piece to generate moves

    let diagonal_moves = get_moves_in_line(board, square, piece, BISHOP_OFFSETS.to_vec());
    let diagonal_attackers: Vec<Move> = diagonal_moves
        .into_iter()
        .filter(|m| m.capture.is_some())
        .filter(|m| [PieceKind::Bishop, PieceKind::Queen].contains(&m.capture.unwrap().get_kind()))
        .collect();

    let orthogonal_moves = get_moves_in_line(board, square, piece, ROOK_OFFSETS.to_vec());
    let orthogonal_attackers: Vec<Move> = orthogonal_moves
        .into_iter()
        .filter(|m| m.capture.is_some())
        .filter(|m| [PieceKind::Rook, PieceKind::Queen].contains(&m.capture.unwrap().get_kind()))
        .collect();

    let knight_moves = get_moves_at_offsets(board, square, piece, KNIGHT_OFFSETS.to_vec());
    let knight_attackers: Vec<Move> = knight_moves
        .into_iter()
        .filter(|m| m.capture.is_some())
        .filter(|m| m.capture.unwrap().get_kind() == PieceKind::Knight)
        .collect();

    let king_moves = get_moves_at_offsets(board, square, piece, KING_OFFSETS.to_vec());
    let king_attackers: Vec<Move> = king_moves
        .into_iter()
        .filter(|m| m.capture.is_some())
        .filter(|m| m.capture.unwrap().get_kind() == PieceKind::King)
        .collect();

    let pawn_attack_offsets = match attacked {
        Color::White => WHITE_PAWN_ATTACK_OFFSETS,
        Color::Black => BLACK_PAWN_ATTACK_OFFSETS,
    };
    let pawn_moves = get_moves_at_offsets(board, square, piece, pawn_attack_offsets.to_vec());
    let pawn_attackers: Vec<Move> = pawn_moves
        .into_iter()
        .filter(|m| m.capture.is_some())
        .filter(|m| m.capture.unwrap().get_kind() == PieceKind::Pawn)
        .collect();

    attackers.extend(diagonal_attackers);
    attackers.extend(orthogonal_attackers);
    attackers.extend(knight_attackers);
    attackers.extend(king_attackers);
    attackers.extend(pawn_attackers);

    attackers
        .into_iter()
        .map(|m| (m.capture.unwrap(), m.to))
        .collect()
}

pub fn get_moves_for_piece(board: &Board, square: Square) -> Vec<Move> {
    let mut moves = Vec::new();
    let piece_option = board.get_piece(square);
    if piece_option.is_none() {
        return moves;
    }

    let piece = piece_option.unwrap();
    let kind = piece.get_kind();

    if kind == PieceKind::Pawn {
        moves.extend(get_moves_for_pawn(board, square, piece));
    }

    if kind == PieceKind::Knight {
        moves.extend(get_moves_at_offsets(
            board,
            square,
            piece,
            KNIGHT_OFFSETS.to_vec(),
        ));
    }

    if kind == PieceKind::Bishop {
        moves.extend(get_moves_in_line(
            board,
            square,
            piece,
            BISHOP_OFFSETS.to_vec(),
        ));
    }

    if kind == PieceKind::Rook {
        moves.extend(get_moves_in_line(
            board,
            square,
            piece,
            ROOK_OFFSETS.to_vec(),
        ));
    }

    if kind == PieceKind::Queen {
        moves.extend(get_moves_in_line(
            board,
            square,
            piece,
            QUEEN_OFFSETS.to_vec(),
        ));
    }

    if kind == PieceKind::King {
        moves.extend(get_moves_at_offsets(
            board,
            square,
            piece,
            KING_OFFSETS.to_vec(),
        ));
        moves.extend(get_castling_moves(board, piece.get_color()));
    }

    moves
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{} {}",
            self.piece.to_char(),
            if self.capture.is_some() { "x" } else { "" },
            self.from,
            self.to,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::{Board, piece::Piece, square::Square},
        r#move::{Move, get_moves_for_piece, get_square_attackers},
    };

    mod piece_moves {
        use super::*;

        #[test]
        fn test_moves_for_paws() {
            let board =
                Board::from_fen("rnbqkbnr/p1pppppp/8/1p6/2P5/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

            let moves = get_moves_for_piece(&board, Square { file: 1, rank: 3 });

            assert_eq!(moves.len(), 2);
            assert!(moves.iter().any(|m| m
                == &Move::new(
                    Square { file: 1, rank: 3 },
                    Square { file: 1, rank: 4 },
                    Piece::WHITE_PAWN
                )));
            assert!(moves.iter().any(|m| {
                m == &Move::new(
                    Square { file: 1, rank: 3 },
                    Square { file: 2, rank: 4 },
                    Piece::WHITE_PAWN,
                )
                .with_capture(Piece::BLACK_PAWN)
            }));
        }

        #[test]
        fn test_moves_for_knight() {
            let board =
                Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let moves = get_moves_for_piece(&board, Square { file: 1, rank: 0 });
            assert_eq!(moves.len(), 2);
        }

        #[test]
        fn test_moves_for_bishop() {
            let board =
                Board::from_fen("rnbqk1nr/pppppppp/8/3b4/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let moves = get_moves_for_piece(&board, Square { file: 3, rank: 3 });
            assert_eq!(moves.len(), 8);
        }

        #[test]
        fn test_moves_for_rook() {
            let board = Board::from_fen("r7/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1");
            let moves = get_moves_for_piece(&board, Square { file: 0, rank: 0 });
            assert_eq!(moves.len(), 13);
        }

        #[test]
        fn test_moves_for_queen() {
            let board = Board::from_fen("8/8/8/8/3q4/8/8/8 w KQkq - 0 1");
            let mut moves = get_moves_for_piece(&board, Square { file: 3, rank: 4 });
            moves.sort_by_key(|m| m.to);
            assert_eq!(moves.len(), 27);
        }

        #[test]
        fn test_moves_for_king() {
            let board = Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let moves = get_moves_for_piece(&board, Square { file: 4, rank: 0 });
            assert_eq!(moves.len(), 3);
        }

        #[test]
        fn test_castling_moves() {
            let board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            let moves = get_castling_moves(&board, Color::White);
            assert_eq!(moves.len(), 2);
            assert!(moves.iter().any(|m| m.castling_rook_from_to
                == Some((Square { file: 7, rank: 0 }, Square { file: 5, rank: 0 }))));
            assert!(moves.iter().any(|m| m.castling_rook_from_to
                == Some((Square { file: 0, rank: 0 }, Square { file: 3, rank: 0 }))));
        }

        #[test]
        fn test_no_castling_when_square_attacked() {
            let board = Board::from_fen("6r1/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
            let moves = get_castling_moves(&board, Color::Black);
            println!("{:?}", moves);
            assert_eq!(moves.len(), 1);
            assert!(moves.iter().any(|m| m.castling_rook_from_to
                == Some((Square { file: 0, rank: 7 }, Square { file: 3, rank: 7 }))));
        }
    }

    mod square_attacked {
        use super::*;

        #[test]
        fn test_attacked_by_pawn() {
            let mut board = Board::from_fen("8/8/8/8/8/3P4/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 2, rank: 4 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(attackers.iter().any(|a| a.0.get_kind() == PieceKind::Pawn));
        }

        #[test]
        fn test_attacked_by_knight() {
            let mut board = Board::from_fen("8/8/3N4/8/8/8/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 2, rank: 4 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(
                attackers
                    .iter()
                    .any(|a| a.0.get_kind() == PieceKind::Knight)
            );
        }

        #[test]
        fn test_attacked_by_bishop() {
            let mut board = Board::from_fen("8/8/8/3B4/8/8/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 0, rank: 0 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(
                attackers
                    .iter()
                    .any(|a| a.0.get_kind() == PieceKind::Bishop)
            );
        }

        #[test]
        fn test_attacked_by_rook() {
            let mut board = Board::from_fen("R7/8/8/8/8/8/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 0, rank: 7 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(attackers.iter().any(|a| a.0.get_kind() == PieceKind::Rook));
        }

        #[test]
        fn test_attacked_by_queen() {
            let mut board = Board::from_fen("8/8/8/3Q4/8/8/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 4, rank: 3 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(attackers.iter().any(|a| a.0.get_kind() == PieceKind::Queen));
        }

        #[test]
        fn test_attacked_by_king() {
            let mut board = Board::from_fen("8/8/8/8/3K4/8/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 4, rank: 3 }, Color::White);
            assert_eq!(attackers.len(), 1);
            assert!(attackers.iter().any(|a| a.0.get_kind() == PieceKind::King));
        }

        #[test]
        fn test_attacked_by_multiple_pieces() {
            let mut board = Board::from_fen("8/8/3N4/4R3/8/3P4/8/8 w - - 0 1");
            let attackers =
                get_square_attackers(&mut board, Square { file: 4, rank: 4 }, Color::White);
            assert!(attackers.len() == 3);
        }
    }
}
