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
// The values are in centipawns
static PAWN_TABLE: [[i32; 8]; 8] = [[0, 0, 0, 0, 0, 0, 0, 0],
                                    [50, 50, 50, 50, 50, 50, 50, 50],
                                    [10, 10, 20, 30, 30, 20, 10, 10],
                                    [5, 5, 10, 25, 25, 10, 5, 5],
                                    [0, 0, 0, 20, 20, 0, 0, 0],
                                    [5, -5, -10, 0, 0, -10, -5, 5],
                                    [5, 10, 10, -20, -20, 10, 10, 5],
                                    [0, 0, 0, 0, 0, 0, 0, 0]];

static KNIGHT_TABLE: [[i32; 8]; 8] = [[-50, -40, -30, -30, -30, -30, -40, -50],
                                      [-40, -20, 0, 0, 0, 0, -20, -40],
                                      [-30, 0, 10, 15, 15, 10, 0, -30],
                                      [-30, 5, 15, 20, 20, 15, 5, -30],
                                      [-30, 0, 15, 20, 20, 15, 0, -30],
                                      [-30, 5, 10, 15, 15, 10, 5, -30],
                                      [-40, -20, 0, 5, 5, 0, -20, -40],
                                      [-50, -40, -30, -30, -30, -30, -40, -50]];

static BISHOP_TABLE: [[i32; 8]; 8] = [[-20, -10, -10, -10, -10, -10, -10, -20],
                                      [-10, 0, 0, 0, 0, 0, 0, -10],
                                      [-10, 0, 5, 10, 10, 5, 0, -10],
                                      [-10, 5, 5, 10, 10, 5, 5, -10],
                                      [-10, 0, 10, 10, 10, 10, 0, -10],
                                      [-10, 10, 10, 10, 10, 10, 10, -10],
                                      [-10, 5, 0, 0, 0, 0, 5, -10],
                                      [-20, -10, -10, -10, -10, -10, -10, -20]];

static ROOK_TABLE: [[i32; 8]; 8] = [[0, 0, 0, 0, 0, 0, 0, 0],
                                    [5, 10, 10, 10, 10, 10, 10, 5],
                                    [-5, 0, 0, 0, 0, 0, 0, -5],
                                    [-5, 0, 0, 0, 0, 0, 0, -5],
                                    [-5, 0, 0, 0, 0, 0, 0, -5],
                                    [-5, 0, 0, 0, 0, 0, 0, -5],
                                    [-5, 0, 0, 0, 0, 0, 0, -5],
                                    [0, 0, 0, 5, 5, 0, 0, 0]];

static QUEEN_TABLE: [[i32; 8]; 8] = [[-20, -10, -10,-5, -5, -10, -10, -20],
                                     [-10, 0, 0, 0, 0, 0, 0, -10],
                                     [-10, 0, 5, 5, 5, 5, 0, -10],
                                     [-5, 0, 5, 5, 5, 5, 0, -5],
                                     [-5, 0, 5, 5, 5, 5, 0, -5],
                                     [-10, 5, 5, 5, 5, 5, 0, -10],
                                     [-10, 0, 0, 0, 0, 0, 0, -10],
                                     [-20, -10, -10, -5, -5, -10, -10, -20]];

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

/// uses principle variation search to return the minimax
/// of the given position
pub fn pvs(board: &Board, mut alpha: i32, beta: i32, depth: u8, line: &mut Vec<String>) -> i32 {
    if depth == 0 {
        return evaluate_position(&board.board)
    }
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board.get_piece(Location{rank: rank, file: file}) {
                if p.color == board.active_color {
                    let legal_moves = logic::get_legal_moves(
                        board, Location { rank: rank as u8, file: file as u8 });
                    for i in 0..legal_moves.len() {
                        if let Some(move_loc) = legal_moves.get(i) {
                            let mut newline = Vec::new();
                            let original_loc = Location { rank: rank as u8, file: file as u8 };
                            let new_board = board.after_move(original_loc, *move_loc);
                            let score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline);
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
