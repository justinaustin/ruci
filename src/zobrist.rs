extern crate rand;

use rand::{Rng, thread_rng};

use board::{Board, Location};
use color::Color;
use piece::Type;

// TODO: increase table size to account for castling rights and en passant

const WHITE_PAWN: usize = 0;
const WHITE_BISHOP: usize = 1;
const WHITE_KNIGHT: usize = 2;
const WHITE_ROOK: usize = 3;
const WHITE_QUEEN: usize = 4;
const WHITE_KING: usize = 5;
const BLACK_PAWN: usize = 6;
const BLACK_BISHOP: usize = 7;
const BLACK_KNIGHT: usize = 8;
const BLACK_ROOK: usize = 9;
const BLACK_QUEEN: usize = 10;
const BLACK_KING: usize = 11;

#[derive(Debug)]
pub struct Table {
    table: [[[u64; 12]; 8]; 8]
}

pub struct Entry {
    pub best_move: (Location, Location),
    pub depth: u8,
    pub evaluation: f64,
    pub line: Vec<String>
}

impl Table {
    pub fn new() -> Table {
        let mut zobrist = Table { table: [[[0; 12]; 8]; 8] };
        zobrist.table = [[[0; 12]; 8]; 8];
        let mut rng = rand::thread_rng();
        // fill the table with random numbers
        for rank in 0..8 {
            for file in 0..8 {
                for i in 0..12 {
                    zobrist.table[rank][file][i] = rng.gen();
                }
            }
        }
        zobrist
    }

    pub fn hash(&self, board: &Board) -> u64 {
        let mut hash = 0u64;
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(p) = board.board[rank][file] {
                    match p.piece_type {
                        Type::Pawn => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_PAWN];
                            } else {
                                hash ^= self.table[rank][file][BLACK_PAWN];
                            }
                        },
                        Type::Bishop => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_BISHOP];
                            } else {
                                hash ^= self.table[rank][file][BLACK_BISHOP];
                            }
                        },
                        Type::Knight => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_KNIGHT];
                            } else {
                                hash ^= self.table[rank][file][BLACK_KNIGHT];
                            }
                        },
                        Type::Rook => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_ROOK];
                            } else {
                                hash ^= self.table[rank][file][BLACK_ROOK];
                            }
                        },
                        Type::Queen => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_QUEEN];
                            } else {
                                hash ^= self.table[rank][file][BLACK_QUEEN];
                            }
                        },
                        Type::King => {
                            if p.color == Color::White {
                                hash ^= self.table[rank][file][WHITE_KING];
                            } else {
                                hash ^= self.table[rank][file][BLACK_KING];
                            }
                        },
                    }
                }
            }
        }
        hash
    }

    // TODO
    // pub fn update_hash(board: &Board, current: u64, move
}
