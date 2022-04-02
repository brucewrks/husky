use rand;
use chess::*;
use std::str::FromStr;

mod evaluator;
use crate::evaluator::Evaluator;

// TODO: implement uci interface
fn main() {
    // let position = Board::default().to_string();
    let position = "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0";

    let best_move = best_move(&position);
    println!("Best move: {}", best_move);
}

// Find the best move for the given position
fn best_move(position:&str) -> ChessMove {
    // Setup evaluator and starting board
    let evaluator = Evaluator::new(3, 0);
    let board = Board::from_str(&position).unwrap();

    // Move scoring initialization
    let mut available_moves = MoveGen::new_legal(&board);
    let mut best_move = ChessMove::new(Square::A1, Square::A2, None);
    let mut best_eval = -120;

    if available_moves.len() == 1 {
        return available_moves.next().unwrap();
    }

    println!("Possible moves: {}", available_moves.len());

    // Iterate available moves to find best move by eval
    for mov in available_moves {
        let updated_board = board.make_move_new(mov);
        let eval = evaluator.get_eval(updated_board);

        // Add some randomness to the equation
        if eval > best_eval || eval == best_eval && rand::random() {
            best_move = mov;
            best_eval = eval;
        }

        // Return fast if we find checkmate
        if best_eval >= 12000 {
            println!("Found mate: {}", best_move);
            return best_move;
        }
    }

    println!("{} {}", best_move, best_eval / 100);
    return best_move;
}
