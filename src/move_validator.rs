use crate::board::Board;
use crate::move_::Move;
use crate::types::{Color, PieceType};

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

// Forward push moves only (no captures)
const WHITE_PAWN_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;

        // Single push (one square forward)
        moves[sq] |= bit << 8;

        // Double push (only from rank 2)
        if sq >= 8 && sq < 16 {
            moves[sq] |= bit << 16;
        }
    }
    moves
};

const BLACK_PAWN_PUSHES: [u64; 64] = {
    let mut moves = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;

        // Single push (one square forward)
        moves[sq] |= bit >> 8;

        // Double push (only from rank 7)
        if sq >= 48 && sq < 56 {
            moves[sq] |= bit >> 16;
        }
    }
    moves
};

// Diagonal attack moves only
const WHITE_PAWN_ATTACKS: [u64; 64] = {
    let mut attacks = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;
        if sq % 8 != 0 {  // Not on A-file
            attacks[sq] |= bit << 7;  // Attack left
        }
        if sq % 8 != 7 {  // Not on H-file
            attacks[sq] |= bit << 9;  // Attack right
        }
    }
    attacks
};

const BLACK_PAWN_ATTACKS: [u64; 64] = {
    let mut attacks = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;
        if sq % 8 != 7 {  // Not on H-file
            attacks[sq] |= bit >> 7;  // Attack left
        }
        if sq % 8 != 0 {  // Not on A-file
            attacks[sq] |= bit >> 9;  // Attack right
        }
    }
    attacks
};

// North rays (upward movement)
const NORTH_RAYS: [u64; 64] = {
    let mut rays = [0u64; 64];
    for sq in 0..64 {
        let mut ray = 0u64;
        let mut bit = 1u64 << sq;
        for _ in 0..7 {
            bit = bit << 8; // Move up one rank
            ray |= bit;
        }
        rays[sq] = ray;
    }
    rays
};

// South rays (downward movement)
const SOUTH_RAYS: [u64; 64] = {
    let mut rays = [0u64; 64];
    for sq in 0..64 {
        let mut ray = 0u64;
        let mut bit = 1u64 << sq;
        for _ in 0..7 {
            bit = bit >> 8; // Move down one rank
            ray |= bit;
        }
        rays[sq] = ray;
    }
    rays
};

// East rays (rightward movement)
const EAST_RAYS: [u64; 64] = {
    let mut rays = [0u64; 64];
    for sq in 0..64 {
        let mut ray = 0u64;
        let mut bit = 1u64 << sq;
        let file = sq % 8;
        for _ in 0..(7 - file) { // Stop at the H file
            bit = bit << 1; // Move right one file
            ray |= bit;
        }
        rays[sq] = ray;
    }
    rays
};

// West rays (leftward movement)
const WEST_RAYS: [u64; 64] = {
    let mut rays = [0u64; 64];
    for sq in 0..64 {
        let mut ray = 0u64;
        let mut bit = 1u64 << sq;
        let file = sq % 8;
        for _ in 0..file { // Stop at the A file
            bit = bit >> 1; // Move left one file
            ray |= bit;
        }
        rays[sq] = ray;
    }
    rays
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

    fn get_pseudo_legal_pawn_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        // Get pawns for the current color
        let pawns_bitboard = match color {
            Color::White => board.white_pawns,
            Color::Black => board.black_pawns,
        };

        // Get the bitboard of all pieces
        let all_pieces = board.all_pieces();
        let empty_squares = !all_pieces;

        // Get the bitboard of opponent pieces
        let opponent_pieces = match color {
            Color::White => board.black_pieces(),
            Color::Black => board.white_pieces(),
        };

        let mut moves = Vec::new();
        let mut remaining_pawns = pawns_bitboard;

        // Define promotion ranks
        let promotion_rank = match color {
            Color::White => 0xFF00000000000000, // 8th rank
            Color::Black => 0x00000000000000FF, // 1st rank
        };

        // Process each pawn
        while remaining_pawns != 0 {
            let square = remaining_pawns.trailing_zeros() as u8;
            remaining_pawns &= !(1u64 << square);

            // Get potential push moves for this pawn
            let potential_pushes = match color {
                Color::White => WHITE_PAWN_PUSHES[square as usize],
                Color::Black => BLACK_PAWN_PUSHES[square as usize],
            };

            // Get potential attack moves for this pawn
            let potential_attacks = match color {
                Color::White => WHITE_PAWN_ATTACKS[square as usize],
                Color::Black => BLACK_PAWN_ATTACKS[square as usize],
            };

            // Initialize valid moves bitboard
            let mut valid_moves = 0u64;

            // Process pushes
            // Single push is valid if the target square is empty
            let single_push = match color {
                Color::White => (1u64 << square) << 8,
                Color::Black => (1u64 << square) >> 8,
            };

            if (single_push & empty_squares) != 0 {
                valid_moves |= single_push;

                // Double push is valid if the pawn is on starting rank and both squares ahead are empty
                let double_push = match color {
                    Color::White => single_push << 8,
                    Color::Black => single_push >> 8,
                };

                // Check if pawn is on starting rank
                let is_on_starting_rank = match color {
                    Color::White => square >= 8 && square < 16, // 2nd rank
                    Color::Black => square >= 48 && square < 56, // 7th rank
                };

                if is_on_starting_rank && (double_push & empty_squares) != 0 {
                    valid_moves |= double_push;
                }
            }

            // Process normal captures - can only capture opponent pieces
            valid_moves |= potential_attacks & opponent_pieces;

            // Process en passant captures
            if let Some(ep_square) = board.en_passant_square {
                let ep_bitboard = 1u64 << ep_square;

                // Check if this pawn can capture en passant
                if (potential_attacks & ep_bitboard) != 0 {
                    valid_moves |= ep_bitboard;
                }
            }

            // Convert valid moves to a list of Move objects
            let dest_squares = self.bitboard_to_squareset(valid_moves);

            for dest in dest_squares {
                // Check if this is a promotion move
                let is_promotion = (1u64 << dest) & promotion_rank != 0;

                // Check if this is an en passant capture
                let is_en_passant = board.en_passant_square.map_or(false, |ep| ep == dest);

                if is_promotion {
                    // Generate separate moves for each promotion piece type
                    for promotion_piece in [
                        Some(PieceType::Queen),
                        Some(PieceType::Rook),
                        Some(PieceType::Bishop),
                        Some(PieceType::Knight),
                    ] {
                        moves.push(Move::new(square, dest, promotion_piece, is_en_passant));
                    }
                } else {
                    // Regular move or en passant
                    moves.push(Move::new(square, dest, None, is_en_passant));
                }
            }
        }

        moves
    }

    fn get_pseudo_legal_rook_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        // Get the appropriate rook bitboard and enemy pieces based on color
        let (mut rooks, enemy_pieces) = match color {
            Color::White => (board.white_rooks, board.black_pieces()),
            Color::Black => (board.black_rooks, board.white_pieces()),
        };

        let all_pieces = board.all_pieces();

        // Iterate through each rook position
        while rooks != 0 {
            let from = rooks.trailing_zeros() as u8;
            rooks &= rooks - 1; // Clear the least significant bit

            // Generate moves for each ray direction (north, south, east, west)
            for &ray in &[
                NORTH_RAYS[from as usize],
                SOUTH_RAYS[from as usize],
                EAST_RAYS[from as usize],
                WEST_RAYS[from as usize]
            ] {
                self.add_ray_moves(ray, all_pieces, enemy_pieces, from, &mut moves);
            }
        }

        moves
    }

    // Helper method to add moves along a ray
    fn add_ray_moves(&self, ray: u64, all_pieces: u64, enemy_pieces: u64, from: u8, moves: &mut Vec<Move>) {
        // Find blockers along this ray
        let blockers = ray & all_pieces;

        if blockers == 0 {
            // No blockers - add all squares along the ray as quiet moves
            self.add_moves_to_empty_squares(ray, from, moves);
            return;
        }

        // Find the first blocker in the ray
        let first_blocker_index = self.find_first_blocker_index(ray, blockers);
        if first_blocker_index >= 64 {
            return; // No valid blocker found (shouldn't happen if blockers != 0)
        }

        let first_blocker = 1u64 << first_blocker_index;

        // Add moves to all squares before the first blocker
        let squares_before_blocker = 0u64.wrapping_sub(first_blocker);
        let quiet_moves = ray & !(first_blocker | (ray & squares_before_blocker));

        self.add_moves_to_empty_squares(quiet_moves, from, moves);

        // If the first blocker is an enemy piece, add a capture move
        if (first_blocker & enemy_pieces) != 0 {
            moves.push(Move::new(from, first_blocker_index as u8, None, false));
        }
    }

    // Helper method to find the index of the first blocker on a ray
    fn find_first_blocker_index(ray: u64, blockers: u64, reference_sq: u8) -> u32 {
        let ray_with_blockers = ray & blockers;

        // All moves northward go to higher square indices (e.g., from 10 -> 18, 26, etc.)
        // whereas southward moves go to lower indices (e.g., from 55 -> 47, 39, etc.)
        if reference_sq < ray_with_blockers.trailing_zeros() as u8 {
            // Ray direction is towards higher indices (e.g., North or East)
            ray_with_blockers.trailing_zeros()
        } else {
            // Ray direction is towards lower indices (e.g., South or West)
            63 - ray_with_blockers.leading_zeros()
        }
    }

    // Helper method to add moves to all empty squares on a bitboard
    fn add_moves_to_empty_squares(&self, squares: u64, from: u8, moves: &mut Vec<Move>) {
        let mut dest_squares = squares;
        while dest_squares != 0 {
            let to = dest_squares.trailing_zeros() as u8;
            moves.push(Move::new(from, to, None, false));
            dest_squares &= dest_squares - 1; // Clear the least significant bit
        }
    }

    fn get_pseudo_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        // Add pawn moves
        moves.extend(self.get_pseudo_legal_pawn_moves(board, color));

        // Add knight moves
        moves.extend(self.get_pseudo_legal_knight_moves(board, color));

        // Add rook moves
        moves.extend(self.get_pseudo_legal_rook_moves(board, color));

        // Add king moves
        moves.extend(self.get_pseudo_legal_king_moves(board, color));

        // TODO: Add bishop and queen moves

        moves
    }








    fn bitboard_to_squareset(&self, bitboard: u64) -> Vec<u8> {
        (0..64).filter(|&i| (bitboard & (1u64 << i)) != 0).collect()
    }


}