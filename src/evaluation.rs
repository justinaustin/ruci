use std::cell::RefMut;
use std::collections::HashMap;

use board::{Board, Location};
use color::Color;
use logic;
use piece::{Piece, Type};
use zobrist::{Entry, Table};

// the following tables are taken from
// https://chessprogramming.wikispaces.com/Simplified+evaluation+function
static PAWN_TABLE: [[f64; 8]; 8] = [
    [0.0,   0.0, 0.0, 0.0, 0.0, 0.0,  0.0, 0.0,],
    [0.5,   0.5, 0.5, 0.5, 0.5, 0.5,  0.5, 0.5,],
    [0.1,   0.1, 0.2, 0.3, 0.3, 0.2,  0.1, 0.1,],
    [0.05, 0.05, 0.1,0.25, 0.25,0.1, 0.05,0.05,],
    [0.0,   0.0, 0.0, 0.2, 0.2, 0.0,  0.0, 0.0,],
    [0.05,-0.05,-0.1, 0.0, 0.0,-0.1,-0.05,0.05,],
    [0.05,  0.1, 0.1,-0.2,-0.2, 0.1,  0.1,0.05,],
    [ 0.0,  0.0, 0.0, 0.0, 0.0, 0.0,  0.0, 0.0,],
];

static KNIGHT_TABLE: [[f64; 8]; 8] = [
    [-0.5,-0.4,-0.3,-0.3,-0.3,-0.3,-0.4,-0.5,],
    [-0.4,-0.2, 0.0, 0.0, 0.0, 0.0,-0.2,-0.4,],
    [-0.3, 0.0, 0.1,0.15,0.15, 0.1, 0.0,-0.3,],
    [-0.3,0.05,0.15, 0.2, 0.2,0.15,0.05,-0.3,],
    [-0.3, 0.0,0.15, 0.2, 0.2,0.15, 0.0,-0.3,],
    [-0.3,0.05, 0.1,0.15,0.15, 0.1,0.05,-0.3,],
    [-0.4,-0.2, 0.0,0.05,0.05, 0.0,-0.2,-0.4,],
    [-0.5,-0.4,-0.3,-0.3,-0.3,-0.3,-0.4,-0.5,],
];

static BISHOP_TABLE: [[f64; 8]; 8] = [
    [-0.2,-0.1,-0.1,-0.1,-0.1,-0.1,-0.1,-0.2,],
    [-0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.1,],
    [-0.1, 0.0,0.05, 0.1, 0.1,0.05, 0.0,-0.1,],
    [-0.1,0.05,0.05, 0.1, 0.1,0.05,0.05,-0.1,],
    [-0.1, 0.0, 0.1, 0.1, 0.1, 0.1, 0.0,-0.1,],
    [-0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1,-0.1,],
    [-0.1,0.05, 0.0, 0.0, 0.0, 0.0,0.05,-0.1,],
    [-0.2,-0.1,-0.1,-0.1,-0.1,-0.1,-0.1,-0.2,],
];

static ROOK_TABLE: [[f64; 8]; 8] = [
    [ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,  0.0,],
    [ 0.05,0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.05,],
    [-0.05,0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.05,],
    [-0.05,0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.05,],
    [-0.05,0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.05,],
    [-0.05,0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.05,],
    [-0.05,0.0, 0.0, 0.0, 0.0, 0.0, 0.0,-0.05,],
    [  0.0,0.0, 0.0,0.05,0.05, 0.0, 0.0,  0.0,],
];

static QUEEN_TABLE: [[f64; 8]; 8] = [
    [ -0.2,-0.1,-0.1,-0.05,-0.05,-0.1,-0.1,-0.2,],
    [ -0.1, 0.0, 0.0,  0.0,  0.0, 0.0, 0.0,-0.1,],
    [ -0.1, 0.0,0.05, 0.05, 0.05,0.05, 0.0,-0.1,],
    [-0.05, 0.0,0.05, 0.05, 0.05,0.05, 0.0,-0.05,],
    [  0.0, 0.0,0.05, 0.05, 0.05,0.05, 0.0,-0.05,],
    [ -0.1,0.05,0.05, 0.05, 0.05,0.05, 0.0,-0.1,],
    [ -0.1, 0.0,0.05,  0.0,  0.0, 0.0, 0.0,-0.1,],
    [ -0.2,-0.1,-0.1,-0.05,-0.05,-0.1,-0.1,-0.2,],
];

const KING_WEIGHT: f64 = 200f64;
const QUEEN_WEIGHT: f64 = 9f64;
const ROOK_WEIGHT: f64 = 5f64;
const KNIGHT_WEIGHT: f64 = 3.2f64;
const BISHOP_WEIGHT: f64 = 3.3f64;
const PAWN_WEIGHT: f64 = 1f64;
const MOBILITY_WEIGHT: f64 = 0.1f64;
// const BAD_PAWN_STRUCT_WEIGHT: f64 = -0.5f64;

pub fn evaluate_position(board: &Board) -> f64 {
    let mut king_diff: f64 = 0.0;
    let mut queen_diff: f64 = 0.0;
    let mut rook_diff: f64 = 0.0;
    let mut knight_diff: f64 = 0.0;
    let mut bishop_diff: f64 = 0.0;
    let mut pawn_diff: f64 = 0.0;
    let mut mobility_diff: f64 = 0.0;
    let mut output = 0.0;

    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                match p.piece_type {
                    Type::Pawn => {
                        if p.color == Color::White {
                            pawn_diff += 1.0;
                            output += PAWN_TABLE[7 - rank][file];
                        } else {
                            pawn_diff -= 1.0;
                            output -= PAWN_TABLE[rank][file];
                        }
                    },
                    Type::Bishop => {
                        if p.color == Color::White {
                            bishop_diff += 1.0;
                            output += BISHOP_TABLE[7 - rank][file];
                        } else {
                            bishop_diff -= 1.0;
                            output -= BISHOP_TABLE[rank][file];
                        }
                    },
                    Type::Knight => {
                        if p.color == Color::White {
                            knight_diff += 1.0;
                            output += KNIGHT_TABLE[7 - rank][file];
                        } else {
                            knight_diff -= 1.0;
                            output -= KNIGHT_TABLE[rank][file];
                        }
                    },
                    Type::Rook => {
                        if p.color == Color::White {
                            rook_diff += 1.0;
                            output += ROOK_TABLE[7 - rank][file];
                        } else {
                            rook_diff -= 1.0;
                            output -= ROOK_TABLE[rank][file];
                        }
                    },
                    Type::Queen => {
                        if p.color == Color::White {
                            queen_diff += 1.0;
                            output += QUEEN_TABLE[7 - rank][file];
                        } else {
                            queen_diff -= 1.0;
                            output -= QUEEN_TABLE[rank][file];
                        }
                    },
                    Type::King => {
                        king_diff += if p.color == Color::White { 1.0 } else { -1.0 };
                    },
                }
                let moves = logic::get_legal_moves(
                    &board, Location { rank: rank as u8, file: file as u8 });
                if p.color == Color::White {
                    mobility_diff += moves.len() as f64;
                } else {
                    mobility_diff -= moves.len() as f64;
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
    let mobility_weight = MOBILITY_WEIGHT * mobility_diff;
    output += king_weight + queen_weight + rook_weight + knight_weight + 
        bishop_weight + pawn_weight;// + mobility_weight;
    if board.active_color == Color::Black {
         output *= -1.0
    }
    output
}

/// uses principle variation search to return the minimax
/// of the given position
///
/// TODO: line sometimes becomes a vector with more than depth elements...
pub fn pvs(board: &Board, mut alpha: f64, beta: f64, depth: u8, line: &mut Vec<String>, 
           table: &mut HashMap<u64, Entry>, zobrist: &Table) -> f64 {
    if depth == 0 {
        return evaluate_position(board)
    }
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                if p.color == board.active_color {
                    let legal_moves = logic::get_legal_moves(
                        board, Location { rank: rank as u8, file: file as u8 });
                    for i in 0..legal_moves.len() {
                        if let Some(move_loc) = legal_moves.get(i) {
                            let mut newline = Vec::new();
                            let original_loc = Location { rank: rank as u8, file: file as u8 };
                            let new_board = board.after_move(original_loc, *move_loc);
                            let score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline, table, zobrist);
                            // let hash = zobrist.hash(&new_board);
                            // let mut score = 0.0;
                            // if table.contains_key(&hash) {
                            //     let e_depth = table.get(&hash).unwrap().depth;
                            //     let e_eval = table.get(&hash).unwrap().evaluation;
                            //     if e_depth >= depth - 1 {
                            //         score = e_eval;
                            //         newline = table.get(&hash).unwrap().line.clone();
                            //     } else {
                            //         score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline, table, zobrist);
                            //         table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
                            //         depth: depth - 1, evaluation: score, line: line.clone() });
                            //     }
                            // } else {
                            //     score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline, table, zobrist);
                            //     table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
                            //     depth: depth - 1, evaluation: score, line: line.clone() });
                            // }
                            if score >= beta {
                                return beta
                            }
                            if score > alpha {
                                alpha = score;
                                line.clear();
                                line.push(original_loc.to_notation());
                                line.push(move_loc.to_notation());
                                line.push(" ".to_owned());
                                for m in &newline {
                                    line.push(m.clone());
                                }
                                // let e_depth = table.get(&hash).unwrap().depth;
                                // let e_eval = table.get(&hash).unwrap().evaluation;
                                // table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
                                // depth: e_depth, evaluation: e_eval, line: line.clone() });
                            }
                        }
                    }
                }
            }
        }
    }
    alpha
}

/// performs a quiescence search on the given position
/// used to evaluate 'quiet' positions
fn quiescence(board: &Board, mut alpha: f64, beta: f64) -> f64 {
    let evaluation = evaluate_position(board);
    if evaluation >= beta {
        return beta
    }
    if alpha < evaluation {
        alpha = evaluation
    }

    // examime every capture
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                if p.color == board.active_color {
                    let legal_moves = logic::get_legal_moves(
                        board, Location { rank: rank as u8, file: file as u8 });
                    for i in 0..legal_moves.len() {
                        if let Some(move_loc) = legal_moves.get(i) {
                            if let Some(other_p) = 
                                board.board[move_loc.rank as usize][move_loc.file as usize] {
                                    if p.color != other_p.color {
                                        let original_loc = Location { rank: rank as u8, file: file as u8 };
                                        let new_board = board.after_move(original_loc, *move_loc);
                                        let score = -quiescence(&new_board, -beta, -alpha);
                                        if score >= beta {
                                            return beta
                                        }
                                        if score > alpha {
                                            alpha = score;
                                        }
                                    }
                                }
                        }
                    }
                }
            }
        }
    }
    alpha
}
