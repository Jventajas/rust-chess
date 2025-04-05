#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub(crate) enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

pub(crate) enum GameResult {
    Draw,
    WhiteWin,
    BlackWin,
}

enum GameStatus {
    Normal,
    Check,
    Checkmate,
    Stalemate,
}
