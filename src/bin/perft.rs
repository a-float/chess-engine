use std::{env, time::SystemTime};

use chess_engine::Board;

#[derive(Default)]
struct Results {
    pub total: u64,
    pub captures: u64,
    pub en_passants: u64,
    pub castles: u64,
}

fn perft(depth: u8, res: &mut Results, board: &mut Board) {
    if depth == 0 {
        res.total += 1;
        return;
    }

    let moves = board.get_legal_moves();
    for m in moves {
        if depth == 1 {
            res.total += 1;
            if m.capture.is_some() {
                res.captures += 1;
            }
            if m.capture.is_some() && m.en_passant_square.is_some() {
                res.en_passants += 1;
            }
            if m.castling_rook_from_to.is_some() {
                res.castles += 1;
            }
        } else {
            board.apply_move(&m);
            perft(depth - 1, res, board);
            board.undo_move(&m);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_depth: u8 = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .expect("Please provide a valid depth argument.");

    if max_depth as usize >= EXPECTED.len() {
        panic!(
            "Provided depth is too big. Maximum supported depth is {}.",
            EXPECTED.len()
        )
    }

    let mut board = Board::default();
    println!(
        "| Depth | Nodes                   | Captures                | En passant        | Castles         | Time (ms) | Nodes/s   |"
    );
    println!(
        "|:-----:|------------------------:|------------------------:|------------------:|----------------:|----------:|----------:|"
    );

    for depth in 1..=max_depth {
        let mut res = Results::default();
        let start = SystemTime::now();
        perft(depth, &mut res, &mut board);
        let end = SystemTime::now();
        let elapsed = end.duration_since(start).unwrap();

        let nodes_per_sec = if elapsed.as_nanos() > 0 {
            (res.total as f64 / (elapsed.as_nanos() as f64 / 1e9)) as u64
        } else {
            0
        };

        let expected = &EXPECTED[depth as usize];
        let nodes_diff = res.total as i64 - expected.total as i64;
        let nodes_str = format!("{} ({:+})", res.total, nodes_diff);

        let captures_diff = res.captures as i64 - expected.captures as i64;
        let captures_str = format!("{} ({:+})", res.captures, captures_diff);

        let ep_diff = res.en_passants as i64 - expected.en_passants as i64;
        let ep_str = format!("{} ({:+})", res.en_passants, ep_diff);

        let castles_diff = res.castles as i64 - expected.castles as i64;
        let castles_str = format!("{} ({:+})", res.castles, castles_diff);

        let nps_str = format_nodes_per_sec(nodes_per_sec);

        println!(
            "| {:5} | {:>23} | {:>23} | {:>17} | {:>15} | {:9} | {:>9} |",
            depth,
            nodes_str,
            captures_str,
            ep_str,
            castles_str,
            elapsed.as_millis(),
            nps_str,
        );
    }
}

fn format_nodes_per_sec(nps: u64) -> String {
    if nps >= 1_000_000 {
        format!("{:.2}M", nps as f64 / 1_000_000.0)
    } else if nps >= 1_000 {
        format!("{:.2}K", nps as f64 / 1_000.0)
    } else {
        format!("{}", nps)
    }
}

static EXPECTED: [Results; 9] = [
    Results {
        total: 1,
        captures: 0,
        en_passants: 0,
        castles: 0,
    },
    Results {
        total: 20,
        captures: 0,
        en_passants: 0,
        castles: 0,
    },
    Results {
        total: 400,
        captures: 0,
        en_passants: 0,
        castles: 0,
    },
    Results {
        total: 8_902,
        captures: 34,
        en_passants: 0,
        castles: 0,
    },
    Results {
        total: 197_281,
        captures: 1_576,
        en_passants: 0,
        castles: 0,
    },
    Results {
        total: 4_865_609,
        captures: 82_719,
        en_passants: 258,
        castles: 0,
    },
    Results {
        total: 119_060_324,
        captures: 2_812_008,
        en_passants: 5_248,
        castles: 0,
    },
    Results {
        total: 3_195_901_860,
        captures: 108_329_926,
        en_passants: 319_617,
        castles: 883_453,
    },
    Results {
        total: 84_998_978_956,
        captures: 3_523_740_106,
        en_passants: 7_187_977,
        castles: 23_605_205,
    },
];
