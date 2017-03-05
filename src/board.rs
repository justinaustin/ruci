use bitboard::Bitboard;
use piece::{Piece, Type};
use color::Color;
use std::char;

/// 0 <= file, rank <= 7
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    pub file: u8,
    pub rank: u8
}

impl Location {
    pub fn to_notation(&self) -> String {
        let mut output = "".to_owned();
        match self.file {
            0 => output.push('a'),
            1 => output.push('b'),
            2 => output.push('c'),
            3 => output.push('d'),
            4 => output.push('e'),
            5 => output.push('f'),
            6 => output.push('g'),
            7 => output.push('h'),
            _ => ()
        }
        output.push(char::from_digit((self.rank + 1) as u32, 10).unwrap());
        output
    }

    pub fn parse_notation(s: &str) -> Location {
        let mut output_location = Location { file: 0, rank: 0 };
        let string = s.to_string();
        let chars = string.chars();
        for ch in chars {
            match ch {
                'a' => output_location.file = 0,
                'b' => output_location.file = 1,
                'c' => output_location.file = 2,
                'd' => output_location.file = 3,
                'e' => output_location.file = 4,
                'f' => output_location.file = 5,
                'g' => output_location.file = 6,
                'h' => output_location.file = 7,
                '1' => output_location.rank = 0,
                '2' => output_location.rank = 1,
                '3' => output_location.rank = 2,
                '4' => output_location.rank = 3,
                '5' => output_location.rank = 4,
                '6' => output_location.rank = 5,
                '7' => output_location.rank = 6,
                '8' => output_location.rank = 7,
                _ => panic!("location parse_notation")
            }
        }
        output_location
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CastlingAvailability {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Board {
    pub board: Bitboard,
    pub active_color: Color,
    pub castling_availability: CastlingAvailability,
    pub en_passant_square: Option<Location>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8
}

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut output_board = Board {
            // this will panic if the fen is not well formed
            board: Bitboard::from_fen(fen).unwrap(),
            active_color: Color::White,
            castling_availability: CastlingAvailability {
                white_kingside: false,
                white_queenside: false,
                black_kingside: false,
                black_queenside: false
            },
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 0
        };
        // piece placement, active color, castling availability,
        // en passant target square, halfmove clock, fullmove number
        let split_fen = fen.split_whitespace().collect::<Vec<_>>();
        if split_fen.len() != 6 {
            // TODO: error here
        }
        let piece_placement = split_fen[0];
        let active_color = split_fen[1];
        let castling_availability = split_fen[2];
        let en_passant_target_square = split_fen[3];
        let halfmove_clock = split_fen[4];
        let fullmove_number = split_fen[5];

        if active_color == "b" {
           output_board.active_color = Color::Black;
        }

        let castle = castling_availability.to_string();
        if castle.contains("K") {
            output_board.castling_availability.white_kingside = true;
        }
        if castle.contains("Q") {
            output_board.castling_availability.white_queenside = true;
        }
        if castle.contains("k") {
            output_board.castling_availability.black_kingside = true;
        }
        if castle.contains("q") {
            output_board.castling_availability.black_queenside = true;
        }

        if en_passant_target_square != "-" {
            output_board.en_passant_square = Some
                (Location::parse_notation(en_passant_target_square));
        }

        let halfmove_clock_string = halfmove_clock.to_string();
        let halfmove = halfmove_clock_string.parse::<u8>().unwrap();
        output_board.halfmove_clock = halfmove;

        let fullmove_number_string = fullmove_number.to_string();
        let fullmove = fullmove_number_string.parse::<u8>().unwrap();
        output_board.fullmove_number = fullmove;

        output_board
    }

    // assumes the move is legal
    // TODO: update for castling, en passant, etc
    pub fn after_move(&self, start: Location, end: Location) -> Board {
        let mut new_board = self.clone();
        new_board.board.after_move(start, end);
        new_board.active_color =
            if new_board.active_color == Color::White {Color::Black} else {Color::White};
        // pawn promotion
        new_board.board.promote_pawns();
        // castling
        if new_board.board.check_kingside_castle(&self.board, self.active_color == Color::White) ||
            new_board.board.check_queenside_castle(&self.board, self.active_color== Color::White) {
            if self.active_color == Color::White {
                new_board.castling_availability.white_kingside = false;
                new_board.castling_availability.white_queenside = false;
            } else {
                new_board.castling_availability.black_kingside = false;
                new_board.castling_availability.black_queenside = false;
            }
        } 
        // update castling availability if rook moved
        if new_board.board.check_rook_move_castling_kingside(true) {
            new_board.castling_availability.white_kingside = false;
        } else if new_board.board.check_rook_move_castling_kingside(false) {
            new_board.castling_availability.black_kingside = false;
        } else if new_board.board.check_rook_move_castling_queenside(true) {
            new_board.castling_availability.white_queenside = false;
        } else if new_board.board.check_rook_move_castling_queenside(false) {
            new_board.castling_availability.black_queenside = false;
        }

        // TODO:en passant
        

//             // en passant
//             if let Some(square) = new_board.en_passant_square {
//                 if p.piece_type == Type::Pawn && end == square {
//                     // capture the en passant pawn
//                     if p.color == Color::White {
//                         new_board.board[end.rank as usize - 1][end.file as usize] = None;
//                     } else {
//                         new_board.board[end.rank as usize + 1][end.file as usize] = None;
//                     }
//                 }
//             }
//             new_board.en_passant_square = None;
//             // update the en_passant_square if needed
//             if p.piece_type == Type::Pawn {
//                 if p.color == Color::White {
//                     if end.rank - start.rank == 2 {
//                         new_board.en_passant_square = Some(Location {rank: 2, file: end.file});
//                     }
//                 } else if start.rank - end.rank == 2 {
//                     new_board.en_passant_square = Some(Location {rank: 5, file: end.file});
//                 }
//             }
//         }
        new_board
    }
}
