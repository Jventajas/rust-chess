use crate::board::Board;
use crate::move_::Move;
use crate::types::Color;

const KING_MOVES: [u64; 64] = {
    let mut moves = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;
        moves[sq] = (bit << 8) |                          // Move up
            (bit >> 8) |                                  // Move down
            (bit << 1 & !0x0101010101010101) |            // Move right (prevent A-file to H-file wrap)
            (bit >> 1 & !0x8080808080808080) |            // Move left (prevent H-file to A-file wrap)
            (bit << 9 & !0x8080808080808080) |            // Move up-right
            (bit >> 9 & !0x0101010101010101) |            // Move down-left
            (bit << 7 & !0x0101010101010101) |            // Move up-left
            (bit >> 7 & !0x8080808080808080);             // Move down-right

        // Add castling moves to the starting positions
        if sq == 4 {  // White King starting position (e1)
            // Add kingside castle (g1)
            moves[sq] |= 1u64 << 6;
            // Add queenside castle (c1)
            moves[sq] |= 1u64 << 2;
        }

        if sq == 60 {  // Black King starting position (e8)
            // Add kingside castle (g8)
            moves[sq] |= 1u64 << 62;
            // Add queenside castle (c8)
            moves[sq] |= 1u64 << 58;
        }
    }
    moves
};

const KNIGHT_MOVES: [u64; 64] = {
    let mut moves = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;

        // Knight moves in all 8 directions:
        // Up 2, right 1
        moves[sq] |= (bit << 17) & !0x01010101010101;
        // Up 2, left 1
        moves[sq] |= (bit << 15) & !0x80808080808080;
        // Up 1, right 2
        moves[sq] |= (bit << 10) & !0x0303030303030303;
        // Up 1, left 2
        moves[sq] |= (bit << 6) & !0xC0C0C0C0C0C0C0C0;

        // Down 2, right 1
        moves[sq] |= (bit >> 15) & !0x01010101010101;
        // Down 2, left 1
        moves[sq] |= (bit >> 17) & !0x80808080808080;
        // Down 1, right 2
        moves[sq] |= (bit >> 6) & !0x0303030303030303;
        // Down 1, left 2
        moves[sq] |= (bit >> 10) & !0xC0C0C0C0C0C0C0C0;
    }
    moves
};


pub(crate) struct MoveValidator {

}

impl MoveValidator {

    pub(crate) fn new() -> Self {
        Self {

        }
    }

    pub(crate) fn get_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        self.get_pseudo_legal_moves(board, color)
            .iter()
            .filter(|move_| self.is_move_legal(board, move_))
            .collect()
    }

    pub(crate) fn is_move_legal(&self, board: &Board, move_: &Move) -> bool {
        false
    }

    fn get_pseudo_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        Vec::new()
    }

    fn get_pseudo_legal_king_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let bitboard = match color {
            Color::White => { board.white_king }
            Color::Black => { board.black_king }
        };

        // Get the bitboard of all pieces of the same color
        let own_pieces = match color {
            Color::White => board.white_pieces(),
            Color::Black => board.black_pieces(),
        };

        let king_square = bitboard.trailing_zeros() as u8;

        // Filter out moves that would capture own pieces
        let king_moves_bitboard = KING_MOVES[king_square as usize] & !own_pieces;
        let king_moves_squares = self.bitboard_to_squareset(king_moves_bitboard);

        king_moves_squares
            .iter()
            .map(|sq| {
                // Determine if it's a castling move
                let is_castling = match (king_square, *sq) {
                    (4, 2) | (4, 6) | (60, 58) | (60, 62) => true,
                    _ => false
                };

                Move::new(king_square, *sq, None, is_castling)
            })
            .collect()
    }

    fn get_pseudo_legal_knight_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        // Get the knight bitboard for the current color
        let knights_bitboard = match color {
            Color::White => board.white_knights,
            Color::Black => board.black_knights,
        };

        // Get the bitboard of all pieces of the same color
        let own_pieces = match color {
            Color::White => board.white_pieces(),
            Color::Black => board.black_pieces(),
        };

        let mut moves = Vec::new();

        // For each knight
        let mut remaining_knights = knights_bitboard;
        while remaining_knights != 0 {
            // Get the position of the least significant set bit (first knight)
            let knight_pos = remaining_knights.trailing_zeros() as u8;

            // Clear this bit to move to the next knight in later iterations
            remaining_knights &= !(1u64 << knight_pos);

            // Get all possible knight moves from this position
            let knight_moves_bitboard = KNIGHT_MOVES[knight_pos];

            // Filter out moves that would capture own pieces
            let valid_moves_bitboard = knight_moves_bitboard & !own_pieces;

            // Convert the bitboard to a list of destination squares
            let dest_squares = self.bitboard_to_squareset(valid_moves_bitboard);

            // Create Move objects for each valid destination
            for dest_square in dest_squares {
                moves.push(Move::new(knight_pos, dest_square, None, false));
            }
        }

        moves
    }


    fn bitboard_to_squareset(&self, bitboard: u64) -> Vec<u8> {
        (0..64).filter(|&i| (bitboard & (1u64 << i)) != 0).collect()
    }





}