mod board;
mod r#move;
use board::Board;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);
    println!("{}", board);
    let moves = board.get_moves_for_color(board::piece::Color::White);
    moves.iter().for_each(|m| println!("{}", m));
}
