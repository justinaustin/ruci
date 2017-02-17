/// Implementation of a Bitboard
///
/// For a u64, a1 is the least significant bit,
/// b1 is the second least significant bit, ...,
/// and h8 is the most significant bit

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bitboard {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,

    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64
}

impl Bitboard {
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
                let current_square: u64  = 2u64.pow(((7 - i) * 8 + index) as u32);
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
}

#[cfg(test)]
mod test {
    use bitboard::Bitboard;

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
}
