use crate::{board::Board, r#move::Move};
use rand::{rng, seq::IndexedRandom};

pub struct Search {}

impl Search {
    pub fn random_move(board: &Board) -> Option<Move> {
        let mut rng = rng();
        let moves = board.get_legal_moves();
        moves.choose(&mut rng).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, search::Search};

    #[test]
    fn test_random_move() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/P7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let m = Search::random_move(&board);
        assert!(board.get_legal_moves().contains(&m.unwrap()));
    }
}
