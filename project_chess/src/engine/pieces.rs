//here i will have all the piece data
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn symbol(&self) -> &'static str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => "♙",
            (PieceType::Knight, Color::White) => "♘",
            (PieceType::Bishop, Color::White) => "♗",
            (PieceType::Rook, Color::White) => "♖",
            (PieceType::Queen, Color::White) => "♕",
            (PieceType::King, Color::White) => "♔",

            (PieceType::Pawn, Color::Black) => "♟",
            (PieceType::Knight, Color::Black) => "♞",
            (PieceType::Bishop, Color::Black) => "♝",
            (PieceType::Rook, Color::Black) => "♜",
            (PieceType::Queen, Color::Black) => "♛",
            (PieceType::King, Color::Black) => "♚",
        }
    }
}
