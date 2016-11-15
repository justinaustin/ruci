use color::Color;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Piece {
    pub piece_type: Type,
    pub color: Color
}
