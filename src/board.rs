
pub(crate) struct Board {
    pub(crate) white_pawns: u64,
    pub(crate) white_knights: u64,
    pub(crate) white_bishops: u64,
    pub(crate) white_rooks: u64,
    pub(crate) white_queen: u64,
    pub(crate) white_king: u64,
    pub(crate) black_pawns: u64,
    pub(crate) black_knights: u64,
    pub(crate) black_bishops: u64,
    pub(crate) black_rooks: u64,
    pub(crate) black_queen: u64,
    pub(crate) black_king: u64,
    white_kingside_castling: bool,
    white_queenside_castling: bool,
    black_kingside_castling: bool,
    black_queenside_castling: bool,
    pub(crate) en_passant_square: Option<u8>,
}

impl Board {

    pub(crate) fn new() -> Self {
        Self {
            white_pawns: 0x000000000000FF00,
            white_knights: 0x0000000000000042,
            white_bishops: 0x0000000000000024,
            white_rooks: 0x0000000000000081,
            white_queen: 0x0000000000000008,
            white_king: 0x0000000000000010,
            black_pawns: 0x00FF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks: 0x8100000000000000,
            black_queen: 0x0800000000000000,
            black_king: 0x1000000000000000,
            white_kingside_castling: true,
            white_queenside_castling: true,
            black_kingside_castling: true,
            black_queenside_castling: true,
            en_passant_square: None,
        }
    }
    
    pub(crate) fn white_pieces(&self) -> u64 {
        self.white_pawns | self.white_rooks |self.white_knights | 
        self.white_bishops | self.white_queen | self.white_king
    }

    pub(crate) fn black_pieces(&self) -> u64 {
        self.black_pawns | self.black_rooks | self.black_knights |
        self.black_bishops | self.black_queen | self.black_king
    }

    pub(crate) fn all_pieces(&self) -> u64 {
        self.white_pawns | self.white_rooks | self.white_knights |
        self.white_bishops | self.white_queen | self.white_king |
        self.black_pawns | self.black_rooks | self.black_knights |
        self.black_bishops | self.black_queen | self.black_king
    }

}