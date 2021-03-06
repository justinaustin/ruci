use std::collections::HashMap;
use std::f64;

use board::{Board, Location};
use evaluation;
use zobrist::{Entry, Table};

pub struct State {
    pub board: Board,
    pub hashmap: HashMap<u64, Entry>,
    pub zobrist: Table,
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            hashmap: HashMap::new(),
            zobrist: Table::new(),
        }
    }

    pub fn print_board(&self) {
        self.board.print_board();
    }

    pub fn update_position(&mut self, input: &Vec<&str>) {
        if input[1] == "fen" {
            let mut s = input[2].to_owned();
            s.push_str(" ");
            s.push_str(input[3]);
            s.push_str(" ");
            s.push_str(input[4]);
            s.push_str(" ");
            s.push_str(input[5]);
            s.push_str(" ");
            s.push_str(input[6]);
            s.push_str(" ");
            s.push_str(input[7]);
            self.board = Board::from_fen(&s);
        } else if input[1] == "startpos" {
            self.board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }
        // skip in input until just after the word 'moves'
        let index = input.iter().position(|&r| r == "moves").unwrap_or(input.len());
        for i in (index + 1)..input.len() {
            let m = input[i];
            let mut start = "".to_owned();
            let mut end = "".to_owned();
            // i feel like substring shouldn't be this hard...
            let mut j = 0;
            for ch in m.chars() {
                if j < 2 {
                    start.push(ch);
                } else {
                    end.push(ch);
                }
                j += 1;
            }
            let start_loc = Location::parse_notation(&start);
            let end_loc = Location::parse_notation(&end);
            self.board = self.board.after_move(start_loc, end_loc);
        }
    }

    pub fn go(&mut self) {
        let mut depth = 1;
        let mut best_move = "".to_owned();
        while depth < 5 {
            let mut line = Vec::new();
            let score = evaluation::pvs(&self.board, f64::NEG_INFINITY, f64::INFINITY, depth, 
                                        &mut line, &mut self.hashmap, &self.zobrist) * 100.0;
            print!("info depth {} score cp {:.0} nodes {} time {} pv ", 
                     depth, score, "1", "1");
            for m in &line {
                print!("{}", m);
            }
            println!("");
            best_move = line[0].clone();
            best_move.push_str(&line[1].clone());
            if score.is_infinite() {
                break;
            }
            depth += 1;
        }
        println!("bestmove {}", best_move);
    }

}
