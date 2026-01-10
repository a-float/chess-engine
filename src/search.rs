use crate::{board::Board, r#move::Move};
use rand::{rng, seq::IndexedRandom};

pub struct Search {}

impl Search {
    pub fn random_move(board: &Board) -> Option<Move> {
        let active_color = board.get_active_color();
        let mut rng = rng();
        let moves = board.get_moves_for_color(active_color);
        moves.choose(&mut rng).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, piece::Color},
        search::Search,
    };

    #[test]
    fn test_random_move() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let m = Search::random_move(&board);
        assert!(
            board
                .get_moves_for_color(Color::White)
                .contains(&m.unwrap())
        );
    }
}
