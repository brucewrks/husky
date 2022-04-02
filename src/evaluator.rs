use std::cmp;
use std::time::Instant;
use std::collections::HashMap;

use chess::*;

const HASH_EXACT : u8 = 0;
const HASH_ALPHA : u8 = 1;
const HASH_BETA  : u8 = 2;

struct HashKey {
    depth: u8,
    flag:  u8,
    eval:  i32
}

pub struct Evaluator {
    max_depth: u8,
    start_time: Instant,
    max_duration: u128,
    hash_map: HashMap<u64, HashKey>
}

impl Evaluator {

    pub fn new (max_depth: u8, mut max_duration:u128) -> Evaluator {
        let start_time = Instant::now();

        if max_duration == 0 {
            max_duration = u128::MAX;
        }

        return Evaluator {
            max_depth,
            start_time,
            max_duration,
            hash_map: HashMap::new()
        }
    }

    fn put_hash(&mut self, hash:u64, depth:u8, flag:u8, eval:i32) {
        self.hash_map.insert(
            hash,
            HashKey { depth, flag, eval }
        );
    }

    pub fn get_eval(&mut self, board:Board) -> i32 {
        let is_maximizing = board.side_to_move() == Color::White;
        let result =  self.minimax(0, board, -12000, 12000, is_maximizing);
        return result;
    }

    fn minimax(&mut self, depth:u8, board:Board, alpha:i32, beta:i32, is_maximizing:bool) -> i32 {

        // Final max depth return
        if depth >= self.max_depth {
            let eval = -1 * self.total_eval(board);
            self.put_hash(board.get_hash(), depth, HASH_EXACT, eval);
            return eval;
        }

        // Over duration return
        let duration = Instant::now().duration_since(self.start_time).as_millis();
        if duration > self.max_duration {
            return -1 * self.total_eval(board);
        }

        // Stored board hash return
        let board_hash = board.get_hash();
        if self.hash_map.contains_key(&board_hash) {
            // println!("Found hash {}", board_hash);
            return self.hash_map.get(&board_hash).unwrap().eval;
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let available_moves = MoveGen::new_legal(&board);

        if is_maximizing {
            let mut best_eval = -12000;
            for mov in available_moves {
                let updated_board = board.make_move_new(mov);
                best_eval = cmp::max(best_eval, self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing));

                alpha = cmp::max(alpha, best_eval);
                if beta <= alpha {
                    self.put_hash(board.get_hash(), depth, HASH_ALPHA, best_eval);
                    return best_eval;
                }
            }

            self.put_hash(board.get_hash(), depth, HASH_EXACT, best_eval);
            return best_eval;
        } else {
            let mut best_eval = 12000;
            for mov in available_moves {
                let updated_board = board.make_move_new(mov);
                best_eval = cmp::min(best_eval, self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing));

                beta = cmp::min(beta, best_eval);
                if beta <= alpha {
                    self.put_hash(board.get_hash(), depth, HASH_BETA, best_eval);
                    return best_eval;
                }
            }

            self.put_hash(board.get_hash(), depth, HASH_EXACT, best_eval);
            return best_eval;
        }
    }

    fn total_eval(&self, board:Board) -> i32 {
        if board.status() == BoardStatus::Stalemate {
            return 0;
        }
        if board.status() == BoardStatus::Checkmate {
            if board.side_to_move() == Color::White {
                return 12000;
            }
            return -12000;
        }

        let piece_eval = Evaluator::piece_evaluation(board);
        let king_eval = Evaluator::king_evaluation(board);
        let move_eval = Evaluator::move_evaluation(board);

        // println!("{} {} {}", piece_eval, king_eval, move_eval);

        return piece_eval + move_eval + king_eval;
    }

    fn piece_evaluation(board:Board) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        let white = board.color_combined(Color::White).clone();
        let black = board.color_combined(Color::Black).clone();

        for sq in white {
            match board.piece_on(sq).unwrap() {
                Piece::Pawn => white_score += 100,
                Piece::Knight => white_score += 300,
                Piece::Bishop => white_score += 300,
                Piece::Rook => white_score += 500,
                Piece::Queen => white_score += 900,
                Piece::King => ()
            }
        }

        for sq in black {
            match board.piece_on(sq).unwrap() {
                Piece::Pawn => black_score += 100,
                Piece::Knight => black_score += 300,
                Piece::Bishop => black_score += 300,
                Piece::Rook => black_score += 500,
                Piece::Queen => black_score += 900,
                Piece::King => ()
            }
        }

        return white_score - black_score;
    }

    fn king_evaluation(board:Board) -> i32 {
        let mut white_available = 0;
        let mut black_available = 0;

        let white_sq = board.king_square(Color::White);
        let black_sq = board.king_square(Color::Black);

        let white_squares: [Option<Square>; 4] = [white_sq.up(), white_sq.down(), white_sq.left(), white_sq.right()];
        for sq in white_squares {
            if sq == None {
                continue;
            }

            let king_move = ChessMove::new(white_sq, sq.unwrap(), None);
            if board.legal(king_move) {
                white_available += 100;
            }
        }

        let black_squares: [Option<Square>; 4] = [black_sq.up(), black_sq.down(), black_sq.left(), black_sq.right()];
        for sq in black_squares {
            if sq == None {
                continue;
            }

            let king_move = ChessMove::new(black_sq, sq.unwrap(), None);
            if board.legal(king_move) {
                black_available += 100;
            }
        }

        return (white_available - black_available) / 160;
    }

    fn move_evaluation(board:Board) -> i32 {
        // Can't check move eval if player is in check.
        if board.checkers().to_size(0) > 0 {
            return 0;
        }

        let white_moves;
        let black_moves;

        if board.side_to_move() == Color::White {
            white_moves = MoveGen::new_legal(&board).len() as i32;
            black_moves = MoveGen::new_legal(&board.null_move().unwrap()).len() as i32;
        } else {
            white_moves = MoveGen::new_legal(&board.null_move().unwrap()).len() as i32;
            black_moves = MoveGen::new_legal(&board).len() as i32;
        }

        return (white_moves - black_moves) * 10;
    }
}
