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

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub capture: Option<Piece>,
    pub piece: Piece,
}

fn get_moves_in_line(board: &Board, square: Square, directions: Vec<(i8, i8)>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let piece = board.get_piece(square).unwrap();

    for (file_delta, rank_delta) in directions.iter() {
        let mut target_square_option = square.offset(*file_delta, *rank_delta);
        while let Some(target_square) = target_square_option {
            if let Some(other_piece) = board.get_piece(target_square) {
                if other_piece.get_color() != piece.get_color() {
                    moves.push(Move {
                        from: square,
                        to: target_square,
                        promotion: None,
                        capture: Some(other_piece),
                        piece,
                    });
                }
                break;
            } else {
                moves.push(Move {
                    from: square,
                    to: target_square,
                    promotion: None,
                    capture: None,
                    piece,
                });
            }
            target_square_option = target_square.offset(*file_delta, *rank_delta);
        }
    }
    moves
}

fn get_moves_at_offsets(board: &Board, square: Square, offsets: Vec<(i8, i8)>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let piece = board.get_piece(square);

    for target_square in offsets
        .iter()
        .map(|(file_delta, rank_delta)| square.offset(*file_delta, *rank_delta))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
    {
        let other_piece = board.get_piece(target_square);
        if other_piece.is_none() || other_piece.unwrap().get_color() != piece.unwrap().get_color() {
            moves.push(Move {
                from: square,
                to: target_square,
                promotion: None,
                capture: other_piece,
                piece: piece.unwrap(),
            });
        }
    }
    moves
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
        let dir = match piece.get_color() {
            Color::White => 1,
            Color::Black => -1,
        };
        let forward_square_option = square.offset(0, dir);
        if let Some(forward_square) = forward_square_option
            && board.is_square_empty(forward_square)
        {
            moves.push(Move {
                from: square,
                to: forward_square,
                promotion: None,
                capture: None,
                piece,
            });
        }
        let attack_offsets = [(-1, dir), (1, dir)];
        for attack_square in attack_offsets
            .iter()
            .map(|(file_delta, rank_delta)| square.offset(*file_delta, *rank_delta))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
        {
            let other_piece_option = board.get_piece(attack_square);
            if let Some(other_piece) = other_piece_option
                && other_piece.get_color() != piece.get_color()
            {
                moves.push(Move {
                    from: square,
                    to: attack_square,
                    promotion: None,
                    capture: other_piece_option,
                    piece,
                });
            }
        }
    }

    if kind == PieceKind::Knight {
        moves.extend(get_moves_at_offsets(board, square, KNIGHT_OFFSETS.to_vec()));
    }

    if kind == PieceKind::Bishop {
        moves.extend(get_moves_in_line(board, square, BISHOP_OFFSETS.to_vec()));
    }

    if kind == PieceKind::Rook {
        moves.extend(get_moves_in_line(board, square, ROOK_OFFSETS.to_vec()));
    }

    if kind == PieceKind::Queen {
        moves.extend(get_moves_in_line(board, square, QUEEN_OFFSETS.to_vec()));
    }

    if kind == PieceKind::King {
        moves.extend(get_moves_at_offsets(board, square, KING_OFFSETS.to_vec()));
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

    use crate::{
        board::{Board, piece::Piece, square::Square},
        r#move::{Move, get_moves_for_piece},
    };

    #[test]
    fn test_moves_for_paws() {
        let board = Board::from_fen("rnbqkbnr/p1pppppp/8/1p6/2P5/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let moves = get_moves_for_piece(&board, Square { file: 1, rank: 3 });

        assert_eq!(moves.len(), 2);
        assert!(moves.iter().any(|m| m
            == &Move {
                from: Square { file: 1, rank: 3 },
                to: Square { file: 1, rank: 4 },
                promotion: None,
                capture: None,
                piece: Piece::WHITE_PAWN,
            }));
        assert!(moves.iter().any(|m| m
            == &Move {
                from: Square { file: 1, rank: 3 },
                to: Square { file: 2, rank: 4 },
                promotion: None,
                capture: Some(Piece::BLACK_PAWN),
                piece: Piece::WHITE_PAWN,
            }));
    }

    #[test]
    fn test_moves_for_knight() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let moves = get_moves_for_piece(&board, Square { file: 1, rank: 0 });
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_moves_for_bishop() {
        let board = Board::from_fen("rnbqk1nr/pppppppp/8/3b4/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
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
        moves.iter().for_each(|m| println!("{:?}", m));
        assert_eq!(moves.len(), 27);
    }

    #[test]
    fn test_moves_for_king() {
        let board = Board::from_fen("rnbqkbnr/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let moves = get_moves_for_piece(&board, Square { file: 4, rank: 0 });
        assert_eq!(moves.len(), 3);
    }
}
