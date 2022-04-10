use std::io;

use chess::*;
use crate::evaluator::Evaluator;

pub struct Uci {
    board: Board,
    evaluator: Evaluator,
}

pub fn new(evaluator:Evaluator) -> Uci {
    return Uci {
        board: Board::default(),
        evaluator,
    };
}

impl Uci {
    pub fn parse(&mut self) {
        let stdin = io::stdin();
        let mut line = String::new();

        loop {
            line.clear();
            stdin.read_line(&mut line).unwrap();

            if line.is_empty() {
                break;
            }

            let argv: Vec<&str> = line.split_whitespace().collect();
            let cmd = argv[0].trim();

            match cmd {
                "uci" => self.uci(),
                "setoption" => (),
                "ucinewgame" => (),
                "isready" => self.isready(),
                "position" => self.position(),
                "go" => (),
                "quit" => break,
                _ => println!("Unrecognized command: {}", cmd),
            }
        }
    }

    fn uci(&mut self) {
        println!("id name Husky");
        println!("id author Bruce Caldwell");
        println!("uciok");
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn go(&mut self) {
        println!("...");
    }

    fn position(&mut self) {
        println!("...");
    }
}
