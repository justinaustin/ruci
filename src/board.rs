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
    pub board: [[Option<Piece>; 8]; 8],
    pub active_color: Color,
    pub castling_availability: CastlingAvailability,
    pub en_passant_square: Option<Location>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8
}

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut output_board = Board {
            board: [
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
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

        let ranks = piece_placement.split("/").collect::<Vec<_>>();
        for i in 0..ranks.len() {
            let rank = ranks[i];
            Board::parse_rank(&mut output_board, rank, 7 - i);
        }

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

    fn parse_rank(output_board: &mut Board, rank_str: &str, rank: usize) {
        let rank_string = String::from(rank_str);
        let chars = rank_string.chars();
        let mut new_rank: [Option<Piece>; 8] = [None; 8];
        let mut index = 0;
        for ch in chars {
            match ch {
                // convert ascii into number
                '1'...'8' => index += ch as usize - 49,

                'r' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::Rook}),
                'R' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::Rook}),

                'n' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::Knight}),
                'N' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::Knight}),

                'b' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::Bishop}),
                'B' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::Bishop}),

                'q' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::Queen}),
                'Q' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::Queen}),

                'k' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::King}),
                'K' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::King}),

                'p' => new_rank[index] = Some(Piece {color: Color::Black, piece_type: Type::Pawn}),
                'P' => new_rank[index] = Some(Piece {color: Color::White, piece_type: Type::Pawn}),
                _ => panic!("parse_rank")
            };
            index += 1;
        }
        output_board.board[rank] = new_rank;
    }

    // assumes the move is legal
    // TODO: update for castling, en passant, etc
    pub fn after_move(&self, start: Location, end: Location) -> Board {
        let mut new_board = self.clone();
        if let Some(p) = new_board.board[start.rank as usize][start.file as usize] {
            new_board.board[start.rank as usize][start.file as usize] = None;
            new_board.active_color = 
                if new_board.active_color == Color::White {Color::Black} else {Color::White};
            new_board.board[end.rank as usize][end.file as usize] = Some(p);
            // pawn promotion...auto promotes to queen...need to be flexable
            // though not a high priority
            if p.piece_type == Type::Pawn {
                if end.rank == 7 && p.color == Color::White {
                    new_board.board[end.rank as usize][end.file as usize] = 
                        Some(Piece {color: Color::White, piece_type: Type::Queen});
                } else if end.rank == 0 && p.color == Color::Black {
                    new_board.board[end.rank as usize][end.file as usize] =
                        Some(Piece {color: Color::Black, piece_type: Type::Queen});
                }
            }
            // castling
            if p.piece_type == Type::King {
                // kingside
                if start.file == 4 && end.file == 6 {
                    // move rook
                    let rook = new_board.board[end.rank as usize][7].unwrap();
                    new_board.board[end.rank as usize][7] = None;
                    new_board.board[end.rank as usize][5] = Some(rook);
                } else if start.file == 4 && end.file == 2 {
                    // queenside
                    // move rook
                    let rook = new_board.board[end.rank as usize][0].unwrap();
                    new_board.board[end.rank as usize][0] = None;
                    new_board.board[end.rank as usize][3] = Some(rook);
                }
                if p.color == Color::White {
                    new_board.castling_availability.white_kingside = false;
                    new_board.castling_availability.white_queenside = false;
                } else {
                    new_board.castling_availability.black_kingside = false;
                    new_board.castling_availability.black_queenside = false;
                }
            }
            // update castling availability if rook moved
            if p.piece_type == Type::Rook {
                if start.rank == 0 {
                    if start.file == 7 {
                        new_board.castling_availability.white_kingside = false;
                    } else if start.file == 0 {
                        new_board.castling_availability.white_queenside = false;
                    }
                } else if start.rank == 7 {
                    if start.file == 7 {
                        new_board.castling_availability.black_kingside = false;
                    } else if start.file == 0 {
                        new_board.castling_availability.black_queenside = false;
                    }
                }
            }
            // en passant
            if let Some(square) = new_board.en_passant_square {
                if p.piece_type == Type::Pawn && end == square {
                    // capture the en passant pawn
                    if p.color == Color::White {
                        new_board.board[end.rank as usize - 1][end.file as usize] = None;
                    } else {
                        new_board.board[end.rank as usize + 1][end.file as usize] = None;
                    }
                }
            }
            new_board.en_passant_square = None;
            // update the en_passant_square if needed
            if p.piece_type == Type::Pawn {
                if p.color == Color::White {
                    if end.rank - start.rank == 2 {
                        new_board.en_passant_square = Some(Location {rank: 2, file: end.file});
                    }
                } else if start.rank - end.rank == 2 {
                    new_board.en_passant_square = Some(Location {rank: 5, file: end.file});
                }
            }
        }
        new_board
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let piece = self.board[rank][file];
                match piece {
                    None => print!(" - "),
                    Some(p) => {
                        match p.color {
                            Color::White => {
                                match p.piece_type {
                                    Type::Rook => print!(" R "),
                                    Type::Knight => print!(" N "),
                                    Type::Bishop => print!(" B "),
                                    Type::Queen => print!(" Q "),
                                    Type::King => print!(" K "),
                                    Type::Pawn => print!(" P "),
                                }
                            },
                            Color::Black => {
                                match p.piece_type {
                                    Type::Rook => print!(" r "),
                                    Type::Knight => print!(" n "),
                                    Type::Bishop => print!(" b "),
                                    Type::Queen => print!(" q "),
                                    Type::King => print!(" k "),
                                    Type::Pawn => print!(" p "),
                                }
                            }
                        }
                    }
                }
            }
            println!("");
        }
        match self.active_color {
            Color::White => println!("active color: White"),
            Color::Black => println!("active color: Black")
        }
        print!("castling availability: ");
        if self.castling_availability.white_kingside {
            print!("K");
        }
        if self.castling_availability.white_queenside {
            print!("Q");
        }
        if self.castling_availability.black_kingside {
            print!("k");
        }
        if self.castling_availability.black_queenside {
            print!("q");
        }
        println!("");
        print!("en passant square: ");
        match self.en_passant_square {
            None => println!("None"),
            Some(l) => println!("File: {}, Rank: {}", l.file, l.rank)
        }
        println!("halfmove clock: {}", self.halfmove_clock);
        println!("fullmove number: {}", self.fullmove_number);
        println!("");
    }
}
