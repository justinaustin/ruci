extern crate rand;

use rand::{Rng, thread_rng};

use board::Board;

const WHITE_PAWN: usize = 1;
const WHITE_BISHOP: usize = 2;
const WHITE_KNIGHT: usize = 3;
const WHITE_ROOK: usize = 4;
const WHITE_QUEEN: usize = 5;
const WHITE_KING: usize = 6;
const BLACK_PAWN: usize = 7;
const BLACK_BISHOP: usize = 8;
const BLACK_KNIGHT: usize = 9;
const BLACK_ROOK: usize = 10;
const BLACK_QUEEN: usize = 11;
const BLACK_KING: usize = 12;

struct Table {
    table: [[[u64; 12]; 8]; 8]
}


impl Table {
    pub fn init(&mut self) {
        self.table = [[[0; 12]; 8]; 8];
        let mut rng = rand::thread_rng();
        // fill the table with random numbers
        for rank in 0..8 {
            for file in 0..8 {
                for i in 0..12 {
                    self.table[rank][file][i] = rng.gen();
                }
            }
        }
    }

    pub fn hash(board: &Board) -> u64 {
        // TODO
        0
    }

    // pub fn update_hash(board: &Board, current: u64, move
}
