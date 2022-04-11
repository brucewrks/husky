use std::io;
use std::str::FromStr;

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
            let cmd = argv[0];

            match cmd {
                "uci" => self.uci(),
                "setoption" => (),
                "ucinewgame" => self.ucinewgame(),
                "isready" => self.isready(),
                "position" => self.position(argv),
                "go" => self.go(argv),
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

    fn ucinewgame(&mut self) {
        self.board = Board::default();
        self.evaluator.clear_hash_map();
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn go(&mut self, argv:Vec<&str>) {
        self.evaluator.clear_hash_map();
        let (best_move, _eval) = self.evaluator.best_move(self.board, 1000, true);
        println!("bestmove {}", best_move);
    }

    fn position(&mut self, argv:Vec<&str>) {
        if argv.len() < 2 {
            return;
        }

        let mut index = 2;
        let pos_type = argv[1];

        match pos_type {
            "fen" => {
                let mut fen_str = String::new();

                while index < argv.len() && argv[index].to_lowercase() != "moves" {
                    fen_str.push_str(argv[index]);
                    fen_str.push_str(" ");
                    index += 1;
                }

                self.board = Board::from_str(fen_str.as_str()).unwrap();
            },
            "startpos" => {
                self.board = Board::default();
            },
            _ => (),
        }

        if index < argv.len() && argv[index].to_lowercase() == "moves" {
            index += 1;

            let mut new_board = self.board;

            while index < argv.len() {
                let new_move = ChessMove::from_str(argv[index]).unwrap();
                new_board = new_board.make_move_new(new_move);
                index += 1;
            }

            self.board = new_board;
        }
    }
}
