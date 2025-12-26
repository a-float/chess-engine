use std::fmt::{Display, Formatter};

type File = u8;
type Rank = u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn from_index(idx: u8) -> Option<Self> {
        if idx < 64 {
            Some(Square {
                file: idx % 8,
                rank: idx / 8,
            })
        } else {
            None
        }
    }

    pub fn to_index(&self) -> usize {
        (self.rank * 8 + self.file) as usize
    }

    pub fn offset(&self, file_delta: i8, rank_delta: i8) -> Option<Self> {
        let new_file = self.file as i8 + file_delta;
        let new_rank = self.rank as i8 + rank_delta;

        if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
            Some(Square {
                file: new_file as u8,
                rank: new_rank as u8,
            })
        } else {
            None
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file_char = (b'a' + self.file) as char;
        let rank_char = (b'1' + self.rank) as char;
        write!(f, "{}{}", file_char, rank_char)
    }
}
