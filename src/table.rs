// generates tables consisting of all pseudo legal moves of each piece
// type from every square

use bitboard::Bitboard;
use board::Location;

pub static mut pawn_t_white: [u64; 64] = [0; 64];
pub static mut pawn_t_black: [u64; 64] = [0; 64];
pub static mut pawn_capture_t_white: [u64; 64] = [0; 64];
pub static mut pawn_capture_t_black: [u64; 64] = [0; 64];
pub static mut knight_t: [u64; 64] = [0; 64];
pub static mut bishop_t: [u64; 64] = [0; 64];
pub static mut rook_t: [u64; 64] = [0; 64];
pub static mut queen_t: [u64; 64] = [0; 64];
pub static mut king_t: [u64; 64] = [0; 64];

/// Returns the index of a rank and file
pub fn index(rank: u8, file: u8) -> usize {
    (rank as usize) * 8 + (file as usize)
}

/// Initalizes the piece tables with all pseudo legal moves
/// of each piece type from every square
pub fn init() {
    for rank in 0..8 {
        for file in 0..8 {
            let index = index(rank, file);
            pawn_t_white[index] = get_pawn_moves(rank, file, true);
            pawn_t_black[index] = get_pawn_moves(rank, file, false);
            pawn_capture_t_white[index] = get_pawn_captures(rank, file, true);
            pawn_capture_t_black[index] = get_pawn_captures(rank, file, false);
            knight_t[index] = get_knight_moves(rank, file);
            bishop_t[index] = get_bishop_moves(rank, file);
            rook_t[index] = get_rook_moves(rank, file);
            queen_t[index] = get_queen_moves(rank, file);
            king_t[index] = get_king_moves(rank, file);
        }
    }
}

/// Returns true iff the input location is in bounds
fn bounds(rank: u8, file: u8) -> bool {
    rank < 8 && file < 8
}

/// Returns a u64 representing all pseudo legal moves of a pawn
/// from the input location
fn get_pawn_moves(rank: u8, file: u8, isWhite: bool) -> u64 {
    let mut output = 0;
    if isWhite {
        if rank == 7 {
            return output;
        } else if rank == 2 {
            let two_up = Bitboard::one_hot_square(Location {
                                                      rank: rank + 2,
                                                      file: file,
                                                  });
            output |= two_up;
        }
        let one_up = Bitboard::one_hot_square(Location {
                                                  rank: rank + 1,
                                                  file: file,
                                              });
        output |= one_up;
    } else {
        if rank == 0 {
            return output;
        } else if rank == 5 {
            let two_up = Bitboard::one_hot_square(Location {
                                                      rank: rank - 2,
                                                      file: file,
                                                  });
            output |= two_up;
        }
        let one_up = Bitboard::one_hot_square(Location {
                                                  rank: rank - 1,
                                                  file: file,
                                              });
        output |= one_up;
    }
    output
}

/// Returns a u64 representing all pseudo legal moves of a pawn
/// from the input location
fn get_pawn_captures(rank: u8, file: u8, isWhite: bool) -> u64 {
    let mut output = 0;
    if isWhite {
        if rank == 7 {
            return output;
        } else {
            if file > 0 {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: rank + 1,
                                                       file: file - 1,
                                                   });
            }
            if file < 7 {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: rank + 1,
                                                       file: file + 1,
                                                   });
            }
        }
    } else {
        if rank == 0 {
            return output;
        } else {
            if file > 0 {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: rank - 1,
                                                       file: file - 1,
                                                   });
            }
            if file < 7 {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: rank - 1,
                                                       file: file + 1,
                                                   });
            }
        }
    }
    output
}

/// Returns a u64 representing all pseudo legal moves of a knight
/// from the input location
fn get_knight_moves(rank: u8, file: u8) -> u64 {
    let mut output = 0;
    let mut n_rank = rank + 1;
    let mut n_file = file - 2;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank + 2;
    n_file = file - 1;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank + 2;
    n_file = file + 1;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank + 1;
    n_file = file + 2;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank - 1;
    n_file = file - 2;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank - 2;
    n_file = file - 1;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank - 2;
    n_file = file + 1;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    n_rank = rank - 1;
    n_file = file + 2;
    if bounds(n_rank, n_file) {
        output |= Bitboard::one_hot_square(Location {
                                               rank: n_rank,
                                               file: n_file,
                                           });
    }
    output
}

/// Returns a u64 representing the pseudo legal moves of a bishop
/// from the given position
fn get_bishop_moves(rank: u8, file: u8) -> u64 {
    let mut output = 0;
    for n_rank in 0..8i8 {
        for n_file in 0..8i8 {
            if (rank as i8 - n_rank).abs() == (file as i8 - n_file).abs() && n_rank != rank as i8 {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: n_rank as u8,
                                                       file: n_file as u8,
                                                   });
            }
        }
    }
    output
}

/// Returns a u64 representing the pseudo legal moves of a rook
/// from the given position
fn get_rook_moves(rank: u8, file: u8) -> u64 {
    let mut output = 0;
    for n_rank in 0..8 {
        for n_file in 0..8 {
            if (n_rank == rank) && (n_file != file) {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: n_rank,
                                                       file: n_file,
                                                   });
            } else if (n_file == file) && (n_rank != rank) {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: n_rank,
                                                       file: n_file,
                                                   });
            }
        }
    }
    output
}

/// Returns a u64 representing the pseudo legal moves of a queen
/// from the given position
fn get_queen_moves(rank: u8, file: u8) -> u64 {
    get_bishop_moves(rank, file) | get_rook_moves(rank, file)
}

/// Returns a u64 representing the pseudo legal moves of a king
/// from the given position
fn get_king_moves(rank: u8, file: u8) -> u64 {
    let mut output = 0;
    for n_rank in (rank - 1)..(rank + 2) {
        for n_file in (file - 1)..(file + 2) {
            if bounds(n_rank, n_file) {
                output |= Bitboard::one_hot_square(Location {
                                                       rank: n_rank,
                                                       file: n_file,
                                                   });
            }
        }
    }
    // AND out the kings current position
    output &= !Bitboard::one_hot_square(Location {
                                            rank: rank,
                                            file: file,
                                        });
    output
}
