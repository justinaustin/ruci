use board::{Board, Location};
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
    let mut output = 
        king_weight + queen_weight + rook_weight + knight_weight + bishop_weight + pawn_weight;
    if board.active_color == Color::Black {
         output *= -1.0
    }
    output
}

/// uses principle variation search to return the minimax
/// of the given position
pub fn pvs(board: &Board, mut alpha: f64, beta: f64, depth: u8, line: &mut Vec<String>) -> f64 {
    if depth == 0 {
        return quiescence(board, alpha, beta)
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
                            let mut new_board = board.clone();
                            new_board.board[rank][file] = None;
                            new_board.board[move_loc.rank as usize][move_loc.file as usize] = Some(p);
                            new_board.active_color = 
                                if p.color == Color::White {Color::Black} else {Color::White};
                            let score = -pvs(&new_board, -beta, -alpha, depth - 1, &mut newline);
                            if score >= beta {
                                return beta
                            }
                            if score > alpha {
                                alpha = score;
                                let original_loc = Location { rank: rank as u8, file: file as u8 };
                                line.clear();
                                line.push(original_loc.to_notation());
                                line.push(move_loc.to_notation());
                                line.push(" ".to_owned());
                                for m in &newline {
                                    line.push(m.clone());
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
                                        let mut new_board = board.clone();
                                        new_board.board[rank][file] = None;
                                        new_board.board[move_loc.rank as usize][move_loc.file as usize] = Some(p);
                                        new_board.active_color = other_p.color;
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
