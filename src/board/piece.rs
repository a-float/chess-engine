#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    const fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }

    pub fn get_kind(&self) -> PieceKind {
        self.kind
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn to_char(&self) -> char {
        match *self {
            Piece::BLACK_BISHOP => '♗',
            Piece::BLACK_KING => '♔',
            Piece::BLACK_KNIGHT => '♘',
            Piece::BLACK_PAWN => '♙',
            Piece::BLACK_QUEEN => '♕',
            Piece::BLACK_ROOK => '♖',
            Piece::WHITE_BISHOP => '♝',
            Piece::WHITE_KING => '♚',
            Piece::WHITE_KNIGHT => '♞',
            Piece::WHITE_PAWN => '♟',
            Piece::WHITE_QUEEN => '♛',
            Piece::WHITE_ROOK => '♜',
        }
    }

    pub const WHITE_BISHOP: Piece = Piece::new(Color::White, PieceKind::Bishop);
    pub const WHITE_KING: Piece = Piece::new(Color::White, PieceKind::King);
    pub const WHITE_KNIGHT: Piece = Piece::new(Color::White, PieceKind::Knight);
    pub const WHITE_PAWN: Piece = Piece::new(Color::White, PieceKind::Pawn);
    pub const WHITE_QUEEN: Piece = Piece::new(Color::White, PieceKind::Queen);
    pub const WHITE_ROOK: Piece = Piece::new(Color::White, PieceKind::Rook);

    pub const BLACK_BISHOP: Piece = Piece::new(Color::Black, PieceKind::Bishop);
    pub const BLACK_KING: Piece = Piece::new(Color::Black, PieceKind::King);
    pub const BLACK_KNIGHT: Piece = Piece::new(Color::Black, PieceKind::Knight);
    pub const BLACK_PAWN: Piece = Piece::new(Color::Black, PieceKind::Pawn);
    pub const BLACK_QUEEN: Piece = Piece::new(Color::Black, PieceKind::Queen);
    pub const BLACK_ROOK: Piece = Piece::new(Color::Black, PieceKind::Rook);
}
