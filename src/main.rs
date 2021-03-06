extern crate rand;

mod board;
mod color;
mod evaluation;
mod logic;
mod moves;
mod piece;
mod zobrist;

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use board::Board;
use moves::State;
use zobrist::Table;

fn readline() -> io::Result<String> {
    let mut buffer = String::new();
    try!(io::stdin().read_line(&mut buffer));
    Ok(buffer)
}

fn uci_info() {
    println!("id name ruci");
    println!("id author J. Austin");
    // TODO: add options?
    println!("uciok");
}

fn is_ready() {
    // TODO
    println!("readyok");
}

fn uci_new_game() {
    // TODO
}

fn print_position(input: &Vec<&str>) {
	if input.len() > 1 {
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
			let board = Board::from_fen(&s);
			board.print_board();
		}
	}
}

fn parse_go_command(game_state: Arc<Mutex<State>>) {
    thread::spawn(move || {
        game_state.lock().unwrap().go();
    });
}

fn stop() {
    // TODO
}

fn ponder_hit() {
    // TODO
}

fn evaluate_position(input: &Vec<&str>) {
    if input.len() > 1 {
        let depth = input[1].parse::<u8>().unwrap();
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
        let board = Board::from_fen(&s);
        let mut line = Vec::new();
        let mut table = HashMap::new();
        let zobrist = Table::new();
        println!("eval: {}", evaluation::pvs(&board, -5000.0, 5000.0, depth, 
                                             &mut line, &mut table, &zobrist));
        print!("bestmoves: ");
        for m in line {
            print!("{}", m);
        }
        println!("");
    }
}

fn tests() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board.print_board();
    board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    board.print_board();
    board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
    board.print_board();
    board = Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    board.print_board();
    board = Board::from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 3");
    board.print_board();
    board = Board::from_fen("r1bqk2r/pppp1pbp/2n2np1/1B2p3/4P3/2P2N2/PP1P1PPP/RNBQ1RK1 w kq - 1 6");
    board.print_board();
    board = Board::from_fen("2k4R/8/2K5/8/8/8/8/8 b - - 0 45");
    board.print_board();
}

fn main() {
    let game_state = Arc::new(Mutex::new(State::new()));
    loop {
        let game_state = game_state.clone();
        let input = readline();
        match input {
            Err(_) => println!("error reading input"),
            Ok(string) => {
                let tokens = string.split_whitespace().collect::<Vec<_>>();
                match tokens[0] {
                    "uci" => uci_info(),
                    "isready" => is_ready(),
                    "ucinewgame" => uci_new_game(),
                    "position" => game_state.lock().unwrap().update_position(&tokens),
                    "go" => parse_go_command(game_state),
                    "stop" => stop(),
                    "ponderhit" => ponder_hit(),
                    "test" => tests(),
                    "print" => game_state.lock().unwrap().print_board(),
                    "eval" => evaluate_position(&tokens),
                    "quit" => break,
                    _ => println!("Unknown command: {}", tokens[0])
                }
            }
        }
    }
}
