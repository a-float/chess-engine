use std::fmt::{Display, Formatter};

type File = u8;
type Rank = u8;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn from_string(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }
        let lowercase = s.to_ascii_uppercase();
        let file = lowercase.chars().nth(0).unwrap() as i8 - 'A' as i8;
        let rank = lowercase.chars().nth(1).unwrap() as i8 - '1' as i8;

        Square::new(file, rank)
    }

    pub fn new(file: i8, rank: i8) -> Option<Square> {
        if (0..8).contains(&file) && (0..8).contains(&rank) {
            Some(Square {
                file: file as u8,
                rank: rank as u8,
            })
        } else {
            None
        }
    }

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

        Square::new(new_file, new_rank)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file_char = (b'a' + self.file) as char;
        let rank_char = (b'1' + self.rank) as char;
        write!(f, "{}{}", file_char, rank_char)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::square::Square;

    #[test]
    fn test_square_from_string() {
        let square1 = Square::from_string("A2");
        assert_eq!(square1, Some(Square { file: 0, rank: 1 }));

        let square2 = Square::from_string("H8");
        assert_eq!(square2, Some(Square { file: 7, rank: 7 }));

        let square3 = Square::from_string("d5");
        assert_eq!(square3, Some(Square { file: 3, rank: 4 }));
    }

    #[test]
    fn test_square_from_string_incorrect() {
        assert_eq!(Square::from_string("Z3"), None);
        assert_eq!(Square::from_string("B9"), None);
        assert_eq!(Square::from_string("W10"), None);
        assert_eq!(Square::from_string("banana"), None);
    }
}
