use std::collections::HashMap;

use board::Board;
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
        }
    }

    pub fn go(&mut self) {
        let mut depth = 1;
        let mut line = Vec::new();
        while depth < 5 {
            let score = evaluation::pvs(&self.board, -10000.0, 10000.0, depth, 
                                        &mut line, &mut self.hashmap, &self.zobrist) * 100.0;
            print!("info depth {} score cp {:.0} nodes {} time {} pv ", 
                     depth, score, "1", "1");
            for m in &line {
                print!("{}", m);
            }
            println!("");
            depth += 1;
        }
    }

}

/// takes as input a board and a depth and returns a string
/// representing what the engine thinks as the
/// best move for the position after searching as deep
/// as depth. Ex: "e2e4"
pub fn get_best_move(board: &Board, depth: u8) -> String {
    "e2e4".to_owned()
}
