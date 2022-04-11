use std::str::FromStr;

mod evaluator;
mod uci;

use chess::{ChessMove, Board};
use crate::evaluator::Evaluator;

fn main() {

    // Sanity checks on known mates
    {
        let mut evaluator = Evaluator::new();

        fn do_assert(mov:ChessMove, correct:&str) {
            assert!(format!("{}", mov) == correct);
        }

        // Henry Buckle vs NN, London, 1840
        // 1. Nf6+ gxf6 2. Bxf7#
        let mut position = "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0";
        let mut board = Board::from_str(position).unwrap();

        let (mut best_move, mut _best_eval) = evaluator.best_move(board, 1000, false);
        do_assert(best_move, "d5f6");

        // Paul Morphy vs Duke Isouard, Paris, 1858
        // 1. Qb8+ Nxb8 2. Rd8#
        position = "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 1000, false);
        do_assert(best_move, "b3b8");

        // William Evans vs Alexander MacDonnell, London, 1826
        // 1. Bb5+ c6 2. Qe6+ Qe7 3. Qxe7#
        position = "r3k2r/ppp2Npp/1b5n/4p2b/2B1P2q/BQP2P2/P5PP/RN5K w kq - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 1000, false);
        do_assert(best_move, "c4b5");

        // Adolf Anderssen vs Ernst Falkbeer, Berlin, 1851
        // 1. Re3+ Kxh2 2. Bxf4+ Kh1 3. Rh3#
        position = "8/2p3N1/6p1/5PB1/pp2Rn2/7k/P1p2K1P/3r4 w - - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 1000, false);
        do_assert(best_move, "e4e3");
    }

    let evaluator = Evaluator::new();
    let mut uci_engine = uci::new(evaluator);
    uci_engine.parse();
}
