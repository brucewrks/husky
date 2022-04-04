use rand;
use chess::*;
use std::str::FromStr;
use std::time::Instant;

mod evaluator;
use crate::evaluator::Evaluator;

// TODO: implement uci interface
fn main() {
    let position = Board::default().to_string();
    // let position = "r1b3kr/3pR1p1/ppq4p/5P2/4Q3/B7/P5PP/5RK1 w - - 1 0";

    let best_move = best_move(&position);
    println!("Best move: {}", best_move);
}

// Find the best move for the given position
fn best_move(position:&str) -> ChessMove {
    // Setup evaluator and starting board
    let mut evaluator = Evaluator::new(500);
    let board = Board::from_str(&position).unwrap();

    // Move scoring initialization
    let mut available_moves = MoveGen::new_legal(&board);
    let mut best_move = ChessMove::new(Square::A1, Square::A2, None);
    let mut best_eval = -120;

    if available_moves.len() == 1 {
        return available_moves.next().unwrap();
    }

    println!("Possible moves: {}", available_moves.len());
    let sorted_moves = Evaluator::order_moves(board, available_moves);

    // Iterate available moves to find best move by eval
    for depth in 3..u8::MAX {
        if evaluator.max_duration <= Instant::now().duration_since(evaluator.start_time).as_millis() {
            break;
        }

        for mov in &sorted_moves {
            let updated_board = board.make_move_new(*mov);
            let eval = evaluator.get_eval(updated_board, depth);

            // Add some randomness to the equation
            if eval > best_eval || eval == best_eval && rand::random() {
                best_move = *mov;
                best_eval = eval;
            }

            println!("{} {} {} {}", mov, eval, evaluator.hash_map.len(), depth);

            // Return fast if we find checkmate
            if best_eval >= 12000 {
                println!("Found mate: {}", best_move);
                return best_move;
            }
        }
    }

    println!("{} {}", best_move, best_eval / 100);
    return best_move;
}
