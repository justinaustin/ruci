extern crate rand;

mod bitboard;
mod board;
mod color;
mod evaluation;
mod logic;
mod moves;
mod piece;
mod table;

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use board::Board;
use moves::State;

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
        // println!("eval: {}", evaluation::pvs(&board, -5000.0, 5000.0, depth, 
        //                                      &mut line, &mut table, &zobrist));
        print!("bestmoves: ");
        for m in line {
            print!("{}", m);
        }
        println!("");
    }
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
                    "eval" => evaluate_position(&tokens),
                    "quit" => break,
                    _ => println!("Unknown command: {}", tokens[0])
                }
            }
        }
    }
}
