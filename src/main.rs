use std::str::FromStr;

mod evaluator;
mod uci;

use chess::{ChessMove, Board};
use crate::evaluator::Evaluator;

fn main() {

    // Sanity checks on known mates
    {
        let mut evaluator = Evaluator::new();

        fn go_assert(mov:ChessMove, correct:&str, fen:&str) {
            if format!("{}", mov) != correct {
                panic!("Got the wrong output for a engine sanity check. Expected: {} Got: {}. Position: {}", correct, mov, fen);
            }
        }

        // White to move, Mate in 1
        // 1. Qa3#
        let mut position = "1Bb3BN/R2Pk2r/1Q5B/4q2R/2bN4/4Q1BK/1p6/1bq1R1rb w - - 0 1";
        let mut board = Board::from_str(position).unwrap();
        let (mut best_move, mut _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "e3a3", position);

        // Black to move, Mate in 1.
        // 1... Qg4#
        position = "7r/p3ppk1/3p4/2p1P1Kp/2P5/3PQ1Pq/PP5P/R6R b - - 0 1";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, true);
        go_assert(best_move, "h3g4", position);

        // Paul Morphy vs Duke Isouard, Paris, 1858
        // White to move, Mate in 2.
        // 1. Qb8+ Nxb8 2. Rd8#
        position = "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "b3b8", position);

        // Enrico Paoli vs Jan Foltys, Trencianske Teplice, 1949
        // Black to move, Mate in 2.
        // 1... Bb5+ 2. Nc4 Rd2#
        position = "8/2k2p2/2b3p1/P1p1Np2/1p3b2/1P1K4/5r2/R3R3 b - - 0 1";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, true);
        go_assert(best_move, "c6b5", position);

        // William Evans vs Alexander MacDonnell, London, 1826
        // White to move, Mate in 3.
        // 1. Bb5+ c6 2. Qe6+ Qe7 3. Qxe7#
        position = "r3k2r/ppp2Npp/1b5n/4p2b/2B1P2q/BQP2P2/P5PP/RN5K w kq - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "c4b5", position);

        // Madame de Remusat vs Napoleon I, Paris, 1802
        // Black to move, Mate in 3.
        // 1... Bc5+ 2. Kxc5 Qb6+ 3. Kd5 Qd6#
        position = "r1b1kb1r/pppp1ppp/5q2/4n3/3KP3/2N3PN/PPP4P/R1BQ1B1R b kq - 0 1";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "e4e3", position);

        // Adolf Anderssen vs Jean Dufresne, Berlin, 1852
        // White to move, Mate in 4.
        // 1. Qxd7+ Kxd7 2. Bf5+ Ke8 3. Bd7+ Kf8 4. Bxe7#
        position = "1r2k1r1/pbppnp1p/1b3P2/8/Q7/B1PB1q2/P4PPP/3R2K1 w - - 1 0";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "a4d7", position);

        // Adolf Anderssen vs Jean Dufresne, Berlin, 1852
        // Black to move, Mate in 4.
        // 1... Bxg2 2. Qh8+ Kxh8 3. Bg5 Qxg5 4. Rfe1 Nf3#
        position = "Q7/p1p1q1pk/3p2rp/4n3/3bP3/7b/PP3PPK/R1B2R2 b - - 0 1";
        board = Board::from_str(position).unwrap();
        (best_move, _best_eval) = evaluator.best_move(board, 5000, false);
        go_assert(best_move, "a4d7", position);
    }

    let evaluator = Evaluator::new();
    let mut uci_engine = uci::new(evaluator);
    uci_engine.parse();
}
