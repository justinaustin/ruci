use bitboard::Bitboard;
use board::{Board, Location};
use piece::{Piece, Type};
use color::Color;
use table;

// TODO: pawn promotion

/// Returns true iff the move is a pseudo legal move
fn is_pseudo_legal_move(board: &Board, start: Location, end: Location) -> bool {
    let white_board = board.board.get_white_pieces();
    let black_board = board.board.get_black_pieces();
    let start_one_hot = Bitboard::one_hot_square(start);
    let end_one_hot = Bitboard::one_hot_square(end);
    // check if the correct color piece is at the start location
    if board.active_color == Color::White && white_board & start_one_hot == 0 {
        return false;
    } else if board.active_color == Color::Black && black_board & start_one_hot == 0 {
        return false;
    }
    if board.board.white_pawns & start_one_hot != 0 {
        // TODO: capture OR regular move
        let moves = Table::pawn_t_white[Table::index(start.rank, start.file)];
        let captures = Table::pawn_capture_t_white[Table::index(start.rank, start.file)];
    }
    false
}

pub fn is_valid_move_string(board: &Board, chess_move: &str) -> bool {
    let start_pos: String = chess_move.chars().take(2).collect();
    let end_pos: String = chess_move.chars().skip(2).take(2).collect();
    let start = Location::parse_notation(&start_pos);
    let end = Location::parse_notation(&end_pos);
    is_valid_move(board, start, end)
}

pub fn is_checkmate(board: &Board) -> bool {
    // iterates through every piece and sees if
    // there are any legal moves
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                if p.color == board.active_color {
                    for new_rank in 0..8 {
                        for new_file in 0..8 {
                            let old_loc = Location {
                                rank: rank as u8,
                                file: file as u8,
                            };
                            let new_loc = Location {
                                rank: new_rank as u8,
                                file: new_file as u8,
                            };
                            if is_valid_move(board, old_loc, new_loc) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
    is_king_in_check(board, board.active_color)
}

pub fn is_stalemate(board: &Board) -> bool {
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                if p.color == board.active_color {
                    for new_rank in 0..8 {
                        for new_file in 0..8 {
                            let old_loc = Location {
                                rank: rank as u8,
                                file: file as u8,
                            };
                            let new_loc = Location {
                                rank: new_rank as u8,
                                file: new_file as u8,
                            };
                            if is_valid_move(board, old_loc, new_loc) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
    !is_king_in_check(board, board.active_color)
}

fn is_valid_move(board: &Board, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    if start.file > 7 || start.rank > 7 || end.file > 7 || end.rank > 7 {
        return false;
    }
    match board.board[start.rank as usize][start.file as usize] {
        None => false,
        Some(p) => {
            if p.color != board.active_color {
                return false;
            }
            if would_king_be_in_check(board, p, start, end) {
                return false;
            }
            match p.piece_type {
                Type::Pawn => is_valid_pawn_move(board, p, start, end),
                Type::Bishop => is_valid_bishop_move(board, p, start, end),
                Type::Knight => is_valid_knight_move(board, p, start, end),
                Type::Rook => is_valid_rook_move(board, p, start, end),
                Type::Queen => is_valid_queen_move(board, p, start, end),
                Type::King => is_valid_king_move(board, p, start, end),
            }
        }
    }
}

pub fn get_legal_moves(board: &Board, start: Location) -> Vec<Location> {
    let mut output = Vec::new();
    if let Some(p) = board.board[start.rank as usize][start.file as usize] {
        for rank in 0..8 {
            for file in 0..8 {
                let new_loc = Location {
                    rank: rank,
                    file: file,
                };
                if is_valid_move(board, start, new_loc) {
                    output.push(new_loc);
                }
            }
        }
    }
    output
}

fn would_king_be_in_check(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    // TODO: handle updating castling, en passant, etc
    let mut new_board = board.clone();
    new_board.board[start.rank as usize][start.file as usize] = None;
    new_board.board[end.rank as usize][end.file as usize] = Some(piece);
    is_king_in_check(&new_board, new_board.active_color)
}

fn is_king_in_check(board: &Board, color: Color) -> bool {
    // find the king
    let mut king_location = Location { rank: 0, file: 0 };
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                match p.piece_type {
                    Type::King => {
                        if p.color == color {
                            king_location.rank = rank as u8;
                            king_location.file = file as u8;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    // iterate through all opposite colored pieces and see if they can
    // move to the king's location
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = board.board[rank][file] {
                if p.color != color {
                    let location = Location {
                        rank: rank as u8,
                        file: file as u8,
                    };
                    match p.piece_type {
                        Type::Pawn => {
                            if is_valid_pawn_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                        Type::Bishop => {
                            if is_valid_bishop_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                        Type::Knight => {
                            if is_valid_knight_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                        Type::Rook => {
                            if is_valid_rook_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                        Type::Queen => {
                            if is_valid_queen_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                        Type::King => {
                            if is_valid_king_move(board, p, location, king_location) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn is_valid_pawn_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    // no capture
    if start.file == end.file {
        if board.board[end.rank as usize][end.file as usize] == None {
            // pawns first move can be two spaces
            match piece.color {
                // make sure there's no piece in the way
                Color::White => {
                    if start.rank == 1 && end.rank == 3 {
                        return board.board[2][end.file as usize] == None;
                    }
                }
                Color::Black => {
                    if start.rank == 6 && end.rank == 4 {
                        return board.board[5][end.file as usize] == None;
                    }
                }
            }
            match piece.color {
                Color::White => return end.rank == start.rank + 1,
                Color::Black => return end.rank as i8 == start.rank as i8 - 1,
            }
        }
    } else if ((start.file as i8) - (end.file as i8)).abs() == 1 {
        match board.board[end.rank as usize][end.file as usize] {
            None => {
                // check en passant
                match board.en_passant_square {
                    None => return false,
                    Some(location) => {
                        return end == location && ((start.rank as i8) - (end.rank as i8)).abs() == 1
                    }
                }
            }
            Some(p) => {
                match piece.color {
                    Color::White => return p.color == Color::Black && end.rank == start.rank + 1,
                    Color::Black => {
                        return p.color == Color::White && end.rank as i8 == start.rank as i8 - 1
                    }
                }
            }
        }
    }
    false
}

fn is_valid_bishop_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    let (high_rank, low_rank) = if start.rank > end.rank {
        (start.rank, end.rank)
    } else {
        (end.rank, start.rank)
    };
    let (high_file, low_file) = if start.file > end.file {
        (start.file, end.file)
    } else {
        (end.file, start.file)
    };

    // did it move diagonally?
    if high_rank - low_rank != high_file - low_file {
        return false;
    }

    // check that no pieces are in the path
    if end.rank > start.rank {
        if end.file > start.file {
            // northwest
            let mut file = low_file + 1;
            for rank in (low_rank + 1)..high_rank {
                if board.board[rank as usize][file as usize] != None {
                    return false;
                }
                file = file + 1;
            }
        } else {
            // northeast
            let mut file = high_file - 1;
            for rank in (low_rank + 1)..high_rank {
                if board.board[rank as usize][file as usize] != None {
                    return false;
                }
                file = file - 1;
            }
        }
    } else {
        if end.file > start.file {
            // southwest
            let mut file = low_file + 1;
            for rank in ((low_rank + 1)..high_rank).rev() {
                if board.board[rank as usize][file as usize] != None {
                    return false;
                }
                file = file + 1;
            }
        } else {
            // southeast
            let mut file = high_file - 1;
            for rank in ((low_rank + 1)..high_rank).rev() {
                if board.board[rank as usize][file as usize] != None {
                    return false;
                }
                file = file - 1;
            }
        }
    }

    match board.board[end.rank as usize][end.file as usize] {
        None => return true,
        Some(p) => return piece.color != p.color,
    }
}

fn is_valid_knight_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    let (high_rank, low_rank) = if start.rank > end.rank {
        (start.rank, end.rank)
    } else {
        (end.rank, start.rank)
    };
    let (high_file, low_file) = if start.file > end.file {
        (start.file, end.file)
    } else {
        (end.file, start.file)
    };

    if high_rank - low_rank > 2 || high_file - low_file > 2 {
        return false;
    }
    if high_rank - low_rank == 2 {
        if high_file - low_file == 1 {
            match board.board[end.rank as usize][end.file as usize] {
                None => return true,
                Some(p) => return piece.color != p.color,
            }
        } else {
            return false;
        }
    } else if high_rank - low_rank == 1 {
        if high_file - low_file == 2 {
            match board.board[end.rank as usize][end.file as usize] {
                None => return true,
                Some(p) => return piece.color != p.color,
            }
        } else {
            return false;
        }
    }
    false
}

fn is_valid_rook_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    let (high_rank, low_rank) = if start.rank > end.rank {
        (start.rank, end.rank)
    } else {
        (end.rank, start.rank)
    };
    let (high_file, low_file) = if start.file > end.file {
        (start.file, end.file)
    } else {
        (end.file, start.file)
    };

    if start.rank != end.rank && start.file != end.file {
        return false;
    }

    // check that no pieces are in the path
    if start.rank == end.rank {
        for file in (low_file + 1)..high_file {
            if board.board[start.rank as usize][file as usize] != None {
                return false;
            }
        }
    } else {
        for rank in (low_rank + 1)..high_rank {
            if board.board[rank as usize][start.file as usize] != None {
                return false;
            }
        }
    }
    match board.board[end.rank as usize][end.file as usize] {
        None => return true,
        Some(p) => return piece.color != p.color,
    }
}

fn is_valid_queen_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    is_valid_bishop_move(board, piece, start, end) || is_valid_rook_move(board, piece, start, end)
}

fn is_valid_king_move(board: &Board, piece: Piece, start: Location, end: Location) -> bool {
    if start == end {
        return false;
    }
    let (high_rank, low_rank) = if start.rank > end.rank {
        (start.rank, end.rank)
    } else {
        (end.rank, start.rank)
    };
    let (high_file, low_file) = if start.file > end.file {
        (start.file, end.file)
    } else {
        (end.file, start.file)
    };

    // check castling
    match piece.color {
        Color::White => {
            if end.rank == 0 && end.file == 6 && board.castling_availability.white_kingside {
                if board.board[0][5] == None && board.board[0][6] == None {
                    return !is_king_in_check(board, Color::White) &&
                           !would_king_be_in_check(board,
                                                   piece,
                                                   start,
                                                   Location { rank: 0, file: 5 });
                }
            } else if end.rank == 0 && end.file == 2 &&
                      board.castling_availability.white_queenside {
                if board.board[0][3] == None && board.board[0][2] == None {
                    return !is_king_in_check(board, Color::White) &&
                           !would_king_be_in_check(board,
                                                   piece,
                                                   start,
                                                   Location { rank: 0, file: 3 });
                }
            }
        }
        Color::Black => {
            if end.rank == 7 && end.file == 6 && board.castling_availability.black_kingside {
                if board.board[7][5] == None && board.board[7][6] == None {
                    return !is_king_in_check(board, Color::Black) &&
                           !would_king_be_in_check(board,
                                                   piece,
                                                   start,
                                                   Location { rank: 7, file: 5 });
                }
            } else if end.rank == 7 && end.file == 2 &&
                      board.castling_availability.black_queenside {
                if board.board[7][3] == None && board.board[7][2] == None {
                    return !is_king_in_check(board, Color::Black) &&
                           !would_king_be_in_check(board,
                                                   piece,
                                                   start,
                                                   Location { rank: 7, file: 3 });
                }
            }
        }
    }

    if high_rank - low_rank > 1 || high_file - low_file > 1 {
        return false;
    }
    match board.board[end.rank as usize][end.file as usize] {
        None => return true,
        Some(p) => return piece.color != p.color,
    }
}


#[cfg(test)]
mod test {
    use logic;
    use board::Board;

    #[test]
    fn test_valid_pawn_simple() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(logic::is_valid_move_string(&board, "e2e4"));
        assert!(logic::is_valid_move_string(&board, "a2a3"));
        assert!(!logic::is_valid_move_string(&board, "h7h6"));
        board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert!(logic::is_valid_move_string(&board, "h7h6"));
        assert!(logic::is_valid_move_string(&board, "d7d5"));
    }

    #[test]
    fn test_valid_pawn_capture() {
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
        assert!(logic::is_valid_move_string(&board, "e4d5"));
    }

    #[test]
    fn test_valid_pawn_en_passant() {
        let board = Board::from_fen("rnbqkbnr/p1p1p1pp/3p4/1p2Pp2/3P4/8/PPP2PPP/RNBQKBNR w KQkq f6 0 4");
        assert!(logic::is_valid_move_string(&board, "e5f6"));
    }

    #[test]
    fn test_valid_bishop_simple() {
        let board = Board::from_fen("rnbqkbnr/pppppp1p/6p1/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 1 2");
        assert!(logic::is_valid_move_string(&board, "f8g7"));
        assert!(!logic::is_valid_move_string(&board, "f8b4"));
    }

    #[test]
    fn test_valid_bishop_capture() {
        let board = Board::from_fen("r1bqk1nr/pp1pppbp/2n3p1/2p3B1/3P4/2N2N2/PPP1PPPP/R2QKB1R w KQkq - 2 5");
        assert!(logic::is_valid_move_string(&board, "g5e7"));
    }

    #[test]
    fn test_valid_knight_simple() {
        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/8/7N/PPPPPPPP/RNBQKB1R w KQkq d6 0 2");
        assert!(logic::is_valid_move_string(&board, "h3f4"));
        assert!(!logic::is_valid_move_string(&board, "h3f5"));
    }

    #[test]
    fn test_valid_knight_capture() {
        let board = Board::from_fen("rnbqkbnr/ppp2ppp/8/3pp3/8/2N4N/PPPPPPPP/R1BQKB1R w KQkq e6 0 3");
        assert!(logic::is_valid_move_string(&board, "c3d5"));
        assert!(!logic::is_valid_move_string(&board, "c3e2"));
    }

    #[test]
    fn test_valid_knight_pin() {
        let board = Board::from_fen("r1bqkbnr/ppp2ppp/2n5/1B1Pp3/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 4");
        assert!(!logic::is_valid_move_string(&board, "c6d4"));
        assert!(!logic::is_valid_move_string(&board, "c6b8"));
    }

    #[test]
    fn test_valid_rook_simple() {
        let board = Board::from_fen("r2qk2r/pppbbppp/2n2n2/1B1Pp3/8/5N2/PPPP1PPP/RNBQR1K1 w kq - 5 7");
        assert!(logic::is_valid_move_string(&board, "e1e3"));
        assert!(!logic::is_valid_move_string(&board, "e1e6"));
    }

    #[test]
    fn test_valid_rook_capture() {
        let board = Board::from_fen("3qk2r/1ppbbppp/1rn2n2/pB1Pp3/3P4/N1P1BN2/PP3PPP/R2QR1K1 b k - 2 10");
        assert!(logic::is_valid_move_string(&board, "b6b5"));
    }

    #[test]
    fn test_valid_queen_simple() {
        let board = Board::from_fen("rnb1kbnr/ppp1pppp/8/3q4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 3");
        assert!(logic::is_valid_move_string(&board, "d5d4"));
        assert!(logic::is_valid_move_string(&board, "d5a5"));
        assert!(!logic::is_valid_move_string(&board, "d5f6"));
    }

    #[test]
    fn test_valid_queen_capture() {
        let board = Board::from_fen("r1b1kb1r/ppp1pppp/2n2n2/8/3Q4/5N2/PPP2PPP/RNB1KB1R w KQkq - 1 6");
        assert!(logic::is_valid_move_string(&board, "d4f6"));
        assert!(logic::is_valid_move_string(&board, "d4a7"));
        assert!(!logic::is_valid_move_string(&board, "d4b2"));
    }

    #[test]
    fn test_valid_king_simple() {
        let board = Board::from_fen("r3kb1r/pbp2ppp/5n2/4p3/8/5N2/PPK2PPP/RNB1R3 b kq - 1 10");
        assert!(logic::is_valid_move_string(&board, "e8d8"));
        assert!(logic::is_valid_move_string(&board, "e8d7"));
        assert!(!logic::is_valid_move_string(&board, "e8f7"));
    }

    #[test]
    fn test_valid_king_capture() {
        let board = Board::from_fen("5b1r/Kr3ppp/p2kbn2/2p1p3/7P/PP3NP1/3N1P2/R1B1R3 w - - 1 22");
        assert!(logic::is_valid_move_string(&board, "a7b7"));
        assert!(logic::is_valid_move_string(&board, "a7a6"));
    }

    #[test]
    fn test_valid_king_would_be_in_check() {
        let board = Board::from_fen("5b1r/1K3ppp/4bn2/2pkp3/3R3P/PP3NP1/3N1P2/R1B5 b - - 4 25");
        assert!(!logic::is_valid_move_string(&board, "d5d6"));
        assert!(!logic::is_valid_move_string(&board, "d5c6"));
        assert!(!logic::is_valid_move_string(&board, "d5d4"));
    }

    #[test]
    fn test_valid_king_castle_invalid() {
        let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 3");
        assert!(!logic::is_valid_move_string(&board, "e1c1"));
    }

    #[test]
    fn test_valid_king_castle_kingside() {
        let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4");
        assert!(logic::is_valid_move_string(&board, "e1g1"));
    }

    #[test]
    fn test_valid_king_castle_queenside() {
        let board = Board::from_fen("r3kbnr/pppqpppp/2n5/1B1p4/3P2b1/2N1PN2/PPP2PPP/R1BQK2R b KQkq - 4 5");
        assert!(logic::is_valid_move_string(&board, "e8c8"));
    }

    #[test]
    fn test_valid_king_castle_in_check() {
        let board = Board::from_fen("r1bqk2r/pppp1ppp/2n2n2/4p3/1b2P3/3P1N2/PPP1BPPP/RNBQK2R w KQkq - 1 5");
        assert!(!logic::is_valid_move_string(&board, "e1g1"));
    }

    #[test]
    fn test_valid_king_castle_passing_through_check() {
        let board = Board::from_fen("r3kbnr/pppb1ppp/2nqp3/1B1p2B1/3P4/2N1PN2/PPP2PPP/R2Q1RK1 b kq - 4 7");
        assert!(!logic::is_valid_move_string(&board, "e8c8"));
    }

    #[test]
    fn test_is_checkmate() {
        let mut board = Board::from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 3");
        assert!(logic::is_checkmate(&board));
        board = Board::from_fen("r1b1kbnr/pppp1Npp/8/8/4q3/5n2/PPPPBP1P/RNBQKR2 w Qkq - 0 8");
        assert!(logic::is_checkmate(&board));
    }

    #[test]
    fn test_is_not_checkmate() {
        let board = Board::from_fen("r1b1k2r/ppp2ppp/5n2/2b1P3/4P3/8/PPP2PPP/RNBqK2R w KQkq - 0 1");
        assert!(!logic::is_checkmate(&board));
    }

    #[test]
    fn test_is_stalemate() {
        let board = Board::from_fen("kr6/p7/K7/8/2n5/8/8/8 w - - 22 22");
        assert!(logic::is_stalemate(&board));
    }

    #[test]
    fn test_is_not_stalemate() {
        let board = Board::from_fen("kr6/p7/K7/5n2/8/8/8/8 w - - 22 22");
        assert!(!logic::is_stalemate(&board));
    }
}
