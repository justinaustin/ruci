/// Implementation of a Bitboard
///
/// For a u64, a1 is the least significant bit,
/// b1 is the second least significant bit, ...,
/// and h8 is the most significant bit

use board::Location;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bitboard {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64
}

impl Bitboard {

    /// Returns an empty Bitboard
    pub fn empty() -> Bitboard {
        Bitboard {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0
        }
    }

    /// Returns a 64 bit integer where the bit representing
    /// the input square is 1 and all of the other bits are 0
    pub fn one_hot_square(loc: Location) -> u64 {
        2u64.pow(loc.rank as u32 * 8 + loc.file as u32)
    }

    /// Returns Some of a Bitboard representation of the input
    /// FEN string, or None if the FEN string is invalid. 
    ///
    /// The Bitboard only represents the pieces and their positions.
    /// It does not deal with castling rights, move count, etc. These
    /// are handled by Board.
    pub fn from_fen(fen: &str) -> Option<Bitboard> {
        let mut output = Bitboard::empty();
        let split_fen = fen.split_whitespace().collect::<Vec<_>>();
        if split_fen.len() != 6 {
            return None
        }
        let ranks = split_fen[0].split("/").collect::<Vec<_>>();
        for i in 0..ranks.len() {
            let rank = String::from(ranks[i]);
            let chars = rank.chars();
            let mut index = 0;
            for ch in chars {
                // 64 bit one hot integer where the bit representing the current
                // square is 1
                let current_location = Location { rank: 7 - i as u8, file: index as u8 };
                let current_square: u64  = Bitboard::one_hot_square(current_location);
                match ch {
                    // convert ascii to number
                    // this represents a number of consecutive empty squares
                    '1'...'8' => index += ch as usize - 49,

                    'r' => output.black_rooks |= current_square,
                    'R' => output.white_rooks |= current_square,

                    'n' => output.black_knights |= current_square,
                    'N' => output.white_knights |= current_square,

                    'b' => output.black_bishops |= current_square,
                    'B' => output.white_bishops |= current_square,

                    'q' => output.black_queens |= current_square,
                    'Q' => output.white_queens |= current_square,

                    'k' => output.black_king |= current_square,
                    'K' => output.white_king |= current_square,

                    'p' => output.black_pawns |= current_square,
                    'P' => output.white_pawns |= current_square,

                    _ => return None
                }
                index += 1;
            }
        }
        Some(output)
    }

    /// Moves a piece on the Bitboard from the start position
    /// to the end position
    ///
    /// Assumes there is a piece at the start position and that
    /// the move is legal
    pub fn after_move(&mut self, start_loc: Location, end_loc: Location) {
        let start = Bitboard::one_hot_square(start_loc);
        let end = Bitboard::one_hot_square(end_loc);

        // find what piece is at the start location then AND it out
        // and OR it into end location
        if self.white_pawns & start != 0 {
            self.white_pawns &= !start;
            self.white_pawns |= end;
        } else if self.black_pawns & start != 0 {
            self.black_pawns &= !start;
            self.black_pawns |= end;
        } else if self.white_knights & start != 0 {
            self.white_knights &= !start;
            self.white_knights |= end;
        } else if self.black_knights & start != 0 {
            self.black_knights &= !start;
            self.black_knights |= end;
        } else if self.white_bishops & start != 0 {
            self.white_bishops &= !start;
            self.white_bishops |= end;
        } else if self.black_bishops & start != 0 {
            self.black_bishops &= !start;
            self.black_bishops |= end;
        } else if self.white_rooks & start != 0 {
            self.white_rooks &= !start;
            self.white_rooks |= end;
        } else if self.black_rooks & start != 0 {
            self.black_rooks &= !start;
            self.black_rooks |= end;
        } else if self.white_queens & start != 0 {
            self.white_queens &= !start;
            self.white_queens |= end;
        } else if self.black_queens & start != 0 {
            self.black_queens &= !start;
            self.black_queens |= end;
        } else if self.white_king & start != 0 {
            self.white_king &= !start;
            self.white_king |= end;
        } else if self.black_king & start != 0 {
            self.black_king &= !start;
            self.black_king |= end;
        }
    }

    pub fn get_entire_board(&self) -> u64 {
        let mut board = self.white_pawns;
        board |= self.black_pawns;
        board |= self.white_knights;
        board |= self.black_knights;
        board |= self.white_bishops;
        board |= self.black_bishops;
        board |= self.white_rooks;
        board |= self.black_rooks;
        board |= self.white_queens;
        board |= self.black_queens;
        board |= self.white_king;
        board |= self.black_king;
        board
    }

    pub fn get_white_pieces(&self) -> u64 {
        let mut board = self.white_pawns;
        board |= self.white_knights;
        board |= self.white_bishops;
        board |= self.white_rooks;
        board |= self.white_queens;
        board |= self.white_king;
        board
    }

    pub fn get_black_pieces(&self) -> u64 {
        let mut board = self.black_pawns;
        board |= self.black_knights;
        board |= self.black_bishops;
        board |= self.black_rooks;
        board |= self.black_queens;
        board |= self.black_king;
        board
    }

    /// checks if there are any pawns to promote and
    /// replaces them with queens (needs to be more flexable)
    pub fn promote_pawns(&mut self) {
        let promoted_white_pawns = self.white_pawns >> 56;
        let promoted_black_pawns = self.black_pawns & 0x00000000000000FF;
        if promoted_white_pawns != 0 {
            self.white_queens |= promoted_white_pawns;
            self.white_pawns &= 0x00FFFFFFFFFFFFFF;
        }
        if promoted_black_pawns != 0 {
            self.black_queens |= promoted_black_pawns;
            self.black_pawns &= 0xFFFFFFFFFFFFFF00;
        }
    }

    /// returns if the previous move was a kingside castle
    /// and if it was, moves the rook to the appropriate place
    pub fn check_kingside_castle(&mut self, old_board: &Bitboard, is_white: bool) -> bool {
        if is_white && old_board.white_king == 0x10 && self.white_king == 0x40 {
            self.white_rooks &= 0xFFFFFFFFFFFFFF7F;
            self.white_rooks |= 0x20;
            return true
        } else if !is_white && old_board.black_king == 0x1000000000000000 && self.black_king == 0x4000000000000000 {
            self.black_rooks &= 0x7FFFFFFFFFFFFFFF;
            self.black_rooks |= 0x2000000000000000;
            return true
        }
        false
    }

    /// returns if the previous move was a queenside castle
    /// and if it was, moves the rook to the appropriate place
    pub fn check_queenside_castle(&mut self, old_board: &Bitboard, is_white: bool) -> bool {
        if is_white && old_board.white_king == 0x10 && self.white_king == 0x4 {
            self.white_rooks &= 0xFFFFFFFFFFFFFFFE;
            self.white_rooks |= 0x8;
            return true
        } else if !is_white && old_board.black_king == 0x1000000000000000 && self.black_king == 0x400000000000000 {
            self.black_rooks &= 0xFEFFFFFFFFFFFFFF;
            self.black_rooks |= 0x800000000000000;
            return true
        }
        false
    }

    /// returns if the previous move was a kingside rook move such that
    /// castling kingside is no longer available
    pub fn check_rook_move_castling_kingside(&self, is_white: bool) -> bool {
        (is_white && self.white_rooks & 0x80 == 0) ||
        (!is_white && self.black_rooks & 0x8000000000000000 == 0)
    }

    /// returns if the previous move was a queenside rook move such that
    /// castling queenside is no longer available
    pub fn check_rook_move_castling_queenside(&self, is_white: bool) -> bool {
        (is_white && self.white_rooks & 0x1 == 0) ||
        (!is_white && self.black_rooks & 0x100000000000000 == 0)
    }
}

#[cfg(test)]
mod test {
    use bitboard::Bitboard;
    use bitboard::EntireBitboard;
    use board::Location;

    #[test]
    fn test_empty() {
        let control = Bitboard {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0
        };
        let bitboard = Bitboard::empty();
        assert_eq!(control, bitboard);
    }

    #[test]
    fn test_fen_start() {
        let control = Bitboard {
            white_pawns: 0xFF00,
            white_knights: 0x42,
            white_bishops: 0x24,
            white_rooks: 0x81,
            white_queens: 0x8,
            white_king: 0x10,
            black_pawns: 0xFF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x800000000000000,
            black_king: 0x1000000000000000
        };
        let bitboard = Bitboard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(control, bitboard);
    }

    #[test]
    fn test_fen_smothered_mate() {
        let control = Bitboard {
            white_pawns: 0x1008E700,
            white_knights: 0x30000000000000,
            white_bishops: 0x20000,
            white_rooks: 0x21,
            white_queens: 0x8,
            white_king: 0x40,
            black_pawns: 0xC8210420000000,
            black_knights: 0x200000000000000,
            black_bishops: 0x400000000000000,
            black_rooks: 0x2100000000000000,
            black_queens: 0x4000000000000,
            black_king: 0x4000000000000000
        };
        let bitboard = Bitboard::from_fen("rnb2rk1/2qpNNpp/p4p2/2p5/4Pp2/1B1P4/PPP2PPP/R2Q1RK1 b - - 0 2").unwrap();
        assert_eq!(control, bitboard);
    }

    #[test]
    fn test_after_move_beginning() {
        let control = Bitboard {
            white_pawns: 0x1000EF00,
            white_knights: 0x42,
            white_bishops: 0x24,
            white_rooks: 0x81,
            white_queens: 0x8,
            white_king: 0x10,
            black_pawns: 0xFF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks: 0x8100000000000000,
            black_queens: 0x800000000000000,
            black_king: 0x1000000000000000
        };
        let mut bitboard = Bitboard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let start = Location { rank: 1, file: 4};
        let end = Location { rank: 3, file: 4};
        bitboard.after_move(start, end);
        assert_eq!(control, bitboard);
    }

    #[test]
    fn test_entire_board_beginning() {
        let control = EntireBitboard(0xFFFF00000000FFFF);
        let bitboard = Bitboard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let entire = bitboard.get_entire_board();
        assert_eq!(control, entire);
    }

    #[test]
    fn test_entire_board_two_kings() {
        let control = EntireBitboard(0x100008000000);
        let bitboard = Bitboard::from_fen("8/8/4k3/8/3K4/8/8/8 w - - 3 31").unwrap();
        let entire = bitboard.get_entire_board();
        assert_eq!(control, entire);
    }
}
