use crate::Board;

#[derive(Debug, Default)]
pub struct EDP {
    pub bm: Option<String>, // best move
    pub ce: Option<f32>,    // centipawn evaluation
    pub id: Option<String>, // position identification
    pub c0: Option<String>, // comment
    pub am: Option<String>, // avoid move
}

impl EDP {
    pub fn from_str(edp: &str) -> (Board, Self) {
        let parts: Vec<&str> = edp.splitn(5, ' ').collect();
        let fen = parts
            .clone()
            .into_iter()
            .take(4)
            .collect::<Vec<_>>()
            .join(" ")
            + " 0 1"; // Add default halfmove and fullmove numbers

        let mut ops = EDP::default();
        let board = Board::from_fen(&fen);

        for op in parts[4]
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            let (key, value) = op.split_once(' ').expect("Failed to split operation");

            match key {
                "bm" => ops.bm = Some(value.to_string()),
                "ce" => ops.ce = Some(value.parse().expect("Failed to parse centipawn evaluation")),
                "id" => ops.id = Some(value.to_string()),
                "c0" => ops.c0 = Some(value.to_string()),
                "am" => ops.am = Some(value.to_string()),
                _ => eprintln!("Unknown EDP operation: {}", key),
            }
        }

        (board, ops)
    }
}

#[cfg(test)]
mod tests {
    use super::EDP;

    #[test]
    fn test_edp_parsing() {
        let edp_str = "r1bqkbnr/pppppppp/n7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - bm e4; ce 20; id test123; c0 This is a comment;";
        let (board, ops) = EDP::from_str(edp_str);

        println!("Parsed EDP: {:?}", ops);

        assert_eq!(ops.bm, Some("e4".to_string()));
        assert_eq!(ops.ce, Some(20.0));
        assert_eq!(ops.id, Some("test123".to_string()));
        assert_eq!(ops.c0, Some("This is a comment".to_string()));

        assert_eq!(
            board.to_fen(),
            "r1bqkbnr/pppppppp/n7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1"
        );
    }
}
