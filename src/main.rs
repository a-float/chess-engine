mod board;
mod constants;
mod fen;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = fen::from_fen(fen);
    println!("{}", board);
}
