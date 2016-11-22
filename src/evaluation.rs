use board::Board;
use color::Color;
use logic;
use piece::{Piece, Type};

const KING_WEIGHT: f64 = 200f64;
const QUEEN_WEIGHT: f64 = 9f64;
const ROOK_WEIGHT: f64 = 5f64;
const KNIGHT_WEIGHT: f64 = 3f64;
const BISHOP_WEIGHT: f64 = 3f64;
const PAWN_WEIGHT: f64 = 1f64;
const BAD_PAWN_STRUCT_WEIGHT: f64 = -0.5f64;
const MOBILITY_WEIGHT: f64 = 0.1f64;

pub fn evaluate_position(board: &Board) -> f64 {
    // first element in tuple is white, second is black
    let mut king_diff: f64 = 0.0;
    let mut queen_diff: f64 = 0.0;
    let mut rook_diff: f64 = 0.0;
    let mut knight_diff: f64 = 0.0;
    let mut bishop_diff: f64 = 0.0;
    let mut pawn_diff: f64 = 0.0;

    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                match p.piece_type {
                    Type::Pawn => {
                        pawn_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                    Type::Bishop => {
                        bishop_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                    Type::Knight => {
                        knight_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                    Type::Rook => {
                        rook_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                    Type::Queen => {
                        queen_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                    Type::King => {
                        king_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                }
            }
        }
    }

    let king_weight = KING_WEIGHT * king_diff;
    let queen_weight = QUEEN_WEIGHT * queen_diff;
    let rook_weight = ROOK_WEIGHT * rook_diff;
    let knight_weight = KNIGHT_WEIGHT * knight_diff;
    let bishop_weight = BISHOP_WEIGHT * bishop_diff;
    let pawn_weight = PAWN_WEIGHT * pawn_diff;
    king_weight + queen_weight + rook_weight + knight_weight + bishop_weight + pawn_weight
}
