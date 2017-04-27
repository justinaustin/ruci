use std::cell::RefMut;
use std::collections::HashMap;
use std::f64;

use board::{Board, Location};
use bitboard::Bitboard;
use color::Color;
use logic;
use piece::{Piece, Type};

// the following tables are taken from
// https://chessprogramming.wikispaces.com/Simplified+evaluation+function
//
// TODO: change all floats to ints (x100)
static PAWN_TABLE: [[f64; 8]; 8] = [[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                                    [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
                                    [0.1, 0.1, 0.2, 0.3, 0.3, 0.2, 0.1, 0.1],
                                    [0.05, 0.05, 0.1, 0.25, 0.25, 0.1, 0.05, 0.05],
                                    [0.0, 0.0, 0.0, 0.2, 0.2, 0.0, 0.0, 0.0],
                                    [0.05, -0.05, -0.1, 0.0, 0.0, -0.1, -0.05, 0.05],
                                    [0.05, 0.1, 0.1, -0.2, -0.2, 0.1, 0.1, 0.05],
                                    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]];

static KNIGHT_TABLE: [[f64; 8]; 8] = [[-0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5],
                                      [-0.4, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.4],
                                      [-0.3, 0.0, 0.1, 0.15, 0.15, 0.1, 0.0, -0.3],
                                      [-0.3, 0.05, 0.15, 0.2, 0.2, 0.15, 0.05, -0.3],
                                      [-0.3, 0.0, 0.15, 0.2, 0.2, 0.15, 0.0, -0.3],
                                      [-0.3, 0.05, 0.1, 0.15, 0.15, 0.1, 0.05, -0.3],
                                      [-0.4, -0.2, 0.0, 0.05, 0.05, 0.0, -0.2, -0.4],
                                      [-0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5]];

static BISHOP_TABLE: [[f64; 8]; 8] = [[-0.2, -0.1, -0.1, -0.1, -0.1, -0.1, -0.1, -0.2],
                                      [-0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.1],
                                      [-0.1, 0.0, 0.05, 0.1, 0.1, 0.05, 0.0, -0.1],
                                      [-0.1, 0.05, 0.05, 0.1, 0.1, 0.05, 0.05, -0.1],
                                      [-0.1, 0.0, 0.1, 0.1, 0.1, 0.1, 0.0, -0.1],
                                      [-0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, -0.1],
                                      [-0.1, 0.05, 0.0, 0.0, 0.0, 0.0, 0.05, -0.1],
                                      [-0.2, -0.1, -0.1, -0.1, -0.1, -0.1, -0.1, -0.2]];

static ROOK_TABLE: [[f64; 8]; 8] = [[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                                    [0.05, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.05],
                                    [-0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.05],
                                    [-0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.05],
                                    [-0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.05],
                                    [-0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.05],
                                    [-0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.05],
                                    [0.0, 0.0, 0.0, 0.05, 0.05, 0.0, 0.0, 0.0]];

static QUEEN_TABLE: [[f64; 8]; 8] = [[-0.2, -0.1, -0.1, -0.05, -0.05, -0.1, -0.1, -0.2],
                                     [-0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.1],
                                     [-0.1, 0.0, 0.05, 0.05, 0.05, 0.05, 0.0, -0.1],
                                     [-0.05, 0.0, 0.05, 0.05, 0.05, 0.05, 0.0, -0.05],
                                     [0.0, 0.0, 0.05, 0.05, 0.05, 0.05, 0.0, -0.05],
                                     [-0.1, 0.05, 0.05, 0.05, 0.05, 0.05, 0.0, -0.1],
                                     [-0.1, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0, -0.1],
                                     [-0.2, -0.1, -0.1, -0.05, -0.05, -0.1, -0.1, -0.2]];

const KING_WEIGHT: i32 = 20000;
const QUEEN_WEIGHT: i32 = 900;
const ROOK_WEIGHT: i32 = 500;
const KNIGHT_WEIGHT: i32 = 320;
const BISHOP_WEIGHT: i32 = 330;
const PAWN_WEIGHT: i32 = 100;
const MOBILITY_WEIGHT: i32 = 10;
// const BAD_PAWN_STRUCT_WEIGHT: i32 = -50;

/// returns the evaluation of the position relative to white in centipawns
pub fn evaluate_position(bitboard: &Bitboard) -> i32 {
    let pawn_weight = ((bitboard.white_pawns.count_ones() -
                        bitboard.black_pawns.count_ones()) as i32) *
                      PAWN_WEIGHT;
    let knight_weight = ((bitboard.white_knights.count_ones() -
                          bitboard.black_knights.count_ones()) as i32) *
                        KNIGHT_WEIGHT;
    let bishop_weight = ((bitboard.white_bishops.count_ones() -
                          bitboard.black_bishops.count_ones()) as i32) *
                        BISHOP_WEIGHT;
    let rook_weight = ((bitboard.white_rooks.count_ones() -
                        bitboard.black_rooks.count_ones()) as i32) *
                      ROOK_WEIGHT;
    let queen_weight = ((bitboard.white_queens.count_ones() -
                         bitboard.black_queens.count_ones()) as i32) *
                       QUEEN_WEIGHT;
    let king_weight = ((bitboard.white_king.count_ones() - bitboard.black_king.count_ones()) as
                       i32) *
                      KING_WEIGHT;
    let material_weight = pawn_weight + knight_weight + bishop_weight + rook_weight +
                          queen_weight + king_weight;
    material_weight
}

// /// uses principle variation search to return the minimax
// /// of the given position
// pub fn pvs(board: &Board, mut alpha: f64, beta: f64, depth: u8, line: &mut Vec<String>) -> f64 {
//     if depth == 0 {
//         return evaluate_position(board)
//     }
//     for rank in 0..8 {
//         for file in 0..8 {
//             if let Some(p) = board.board[rank][file] {
//                 if p.color == board.active_color {
//                     let legal_moves = logic::get_legal_moves(
//                         board, Location { rank: rank as u8, file: file as u8 });
//                     for i in 0..legal_moves.len() {
//                         if let Some(move_loc) = legal_moves.get(i) {
//                             let mut newline = Vec::new();
//                             let original_loc = Location { rank: rank as u8, file: file as u8 };
//                             let new_board = board.after_move(original_loc, *move_loc);
//                             let score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline);
//                             // let hash = zobrist.hash(&new_board);
//                             // let mut score = 0.0;
//                             // if table.contains_key(&hash) {
//                             //     let e_depth = table.get(&hash).unwrap().depth;
//                             //     let e_eval = table.get(&hash).unwrap().evaluation;
//                             //     if e_depth >= depth - 1 {
//                             //         score = e_eval;
//                             //         newline = table.get(&hash).unwrap().line.clone();
//                             //     } else {
//                             //         score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline, table, zobrist);
//                             //         table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
//                             //         depth: depth - 1, evaluation: score, line: line.clone() });
//                             //     }
//                             // } else {
//                             //     score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline, table, zobrist);
//                             //     table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
//                             //     depth: depth - 1, evaluation: score, line: line.clone() });
//                             // }

//                             // for checkmate
//                             if score.is_infinite() {
//                                 if board.active_color == Color::White && score > 0.0 {
//                                     line.clear();
//                                     line.push(original_loc.to_notation());
//                                     line.push(move_loc.to_notation());
//                                     line.push(" ".to_owned());
//                                     for m in &newline {
//                                         line.push(m.clone());
//                                     }
//                                     return score;
//                                 } else if board.active_color == Color::Black && score > 0.0 {
//                                     line.clear();
//                                     line.push(original_loc.to_notation());
//                                     line.push(move_loc.to_notation());
//                                     line.push(" ".to_owned());
//                                     for m in &newline {
//                                         line.push(m.clone());
//                                     }
//                                     return score;
//                                 }
//                             }
//                             if score >= beta {
//                                 return beta
//                             }
//                             if score > alpha {
//                                 alpha = score;
//                                 line.clear();
//                                 line.push(original_loc.to_notation());
//                                 line.push(move_loc.to_notation());
//                                 line.push(" ".to_owned());
//                                 for m in &newline {
//                                     line.push(m.clone());
//                                 }
//                                 // let e_depth = table.get(&hash).unwrap().depth;
//                                 // let e_eval = table.get(&hash).unwrap().evaluation;
//                                 // table.insert(hash, Entry { best_move: (original_loc, move_loc.clone()),
//                                 // depth: e_depth, evaluation: e_eval, line: line.clone() });
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     alpha
// }
