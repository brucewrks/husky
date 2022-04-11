use std::cmp;
use std::time::Instant;
use std::collections::HashMap;

use rand;
use chess::*;

const HASH_EXACT : u8 = 0;
const HASH_ALPHA : u8 = 1;
const HASH_BETA  : u8 = 2;

pub struct HashKey {
    depth: u8,
    flag:  u8,
    eval:  i32
}

pub struct Evaluator {
    pub start_time: Instant,
    pub max_duration: u128,
    pub hash_map: HashMap<u64, HashKey>
}

impl Evaluator {
    pub fn new () -> Evaluator {
        let start_time = Instant::now();

        return Evaluator {
            start_time,
            max_duration: u128::MAX,
            hash_map: HashMap::with_capacity(1024)
        }
    }

    pub fn order_moves(board:Board, available_moves:MoveGen) -> Vec<ChessMove> {
        let mut moves:Vec<ChessMove> = available_moves.collect();

        // Favor moves with lower-value pieces
        moves.sort_by(|a, b| board.piece_on(b.get_source()).cmp(&board.piece_on(a.get_source())));

        // Favor moves taking pieces with lower-value pieces
        moves.sort_by(|a, b| board.piece_on(a.get_dest()).cmp(&board.piece_on(b.get_dest())));

        // Favor moves that are checks
        moves.sort_by(|a, b| board.make_move_new(*b).checkers().to_size(0).cmp(&board.make_move_new(*a).checkers().to_size(0)));

        return moves;
    }

    pub fn clear_hash_map(&mut self) {
        self.hash_map.clear();
        assert!(self.hash_map.is_empty());
    }

    fn hash_put(&mut self, hash:u64, depth:u8, flag:u8, eval:i32) {
        self.hash_map.insert(
            hash,
            HashKey { depth, flag, eval }
        );
    }

    pub fn best_move(&mut self, board:Board, max_duration:u128, debug:bool) -> (ChessMove, f32) {
        self.start_time = Instant::now();

        if max_duration == 0 {
            self.max_duration = u128::MAX;
        } else {
            self.max_duration = max_duration;
        }

        // Move scoring initialization
        let mut available_moves = MoveGen::new_legal(&board);
        let mut best_move = ChessMove::new(Square::A1, Square::A2, None);
        let mut best_eval = i32::MIN;

        if board.side_to_move() == Color::Black {
            best_eval = i32::MAX;
        }

        if available_moves.len() == 1 {
            let the_move = available_moves.next().unwrap();
            let the_eval = self.total_eval(board.make_move_new(the_move));
            return (the_move, Evaluator::convert_eval(the_eval));
        }

        let sorted_moves = Evaluator::order_moves(board, available_moves);

        // Iterate available moves to find best move by eval
        for depth in 1..u8::MAX {
            if self.max_duration <= Instant::now().duration_since(self.start_time).as_millis() {
                break;
            }

            if debug {
                println!("info depth {}", depth);
            }

            for mov in &sorted_moves {
                let updated_board = board.make_move_new(*mov);
                let eval = self.get_eval(updated_board, depth);

                if board.side_to_move() == Color::White {
                    if eval > best_eval || eval == best_eval && rand::random() {
                        best_move = *mov;
                        best_eval = eval;
                    }
                } else {
                    if eval < best_eval || eval == best_eval && rand::random() {
                        best_move = *mov;
                        best_eval = eval;
                    }
                }

                // UCI Debugging
                if debug {
                    let pv = format!("{}", mov);
                    println!("info score cp {} depth {} nodes {} time {} pv {}", best_eval, depth, self.hash_map.len(), Instant::now().duration_since(self.start_time).as_millis(), pv);
                }

                // Return fast if we find checkmate
                if best_eval >= 10000 || best_eval <= -10000 {
                    return (best_move, Evaluator::convert_eval(best_eval));
                }
            }
        }

        return (best_move, Evaluator::convert_eval(best_eval));
    }

    fn convert_eval(eval:i32) -> f32 {
        return eval as f32 / 100 as f32;
    }

    pub fn get_eval(&mut self, board:Board, depth:u8) -> i32 {
        let is_maximizing = board.side_to_move() == Color::White;
        let result = self.minimax(0, board, i32::MIN, i32::MAX, is_maximizing, false, depth);
        return result;
    }

    fn minimax(&mut self, depth:u8, board:Board, alpha:i32, beta:i32, is_maximizing:bool, only_captures:bool, max_depth:u8) -> i32 {

        // Final max depth return
        if depth >= max_depth {
            let eval = self.total_eval(board);
            self.hash_put(board.get_hash(), depth, HASH_EXACT, eval);
            return eval;
        }

        // Over duration return
        let duration = Instant::now().duration_since(self.start_time).as_millis();
        if duration > self.max_duration {
            return self.total_eval(board);
        }

        // Stored board hash return
        let board_hash = board.get_hash();
        if self.hash_map.contains_key(&board_hash) {
            let hash = self.hash_map.get(&board_hash).unwrap();
            if hash.depth > depth {
                if hash.flag == HASH_ALPHA && beta <= hash.eval {
                    return hash.eval;
                }
                if hash.flag == HASH_BETA && hash.eval <= alpha {
                    return hash.eval;
                }
                if hash.flag == HASH_EXACT && beta >= hash.eval && alpha <= hash.eval {
                    return hash.eval;
                }
            }
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let mut available_moves = MoveGen::new_legal(&board);

        if only_captures {
            let targets = board.color_combined(!board.side_to_move());
            available_moves.set_iterator_mask(*targets);
        }

        if available_moves.len() == 0 {
            if board.side_to_move() == Color::White {
                return -12000;
            }
            return 12000;
        }

        if is_maximizing {
            let mut best_eval = i32::MIN;
            for mov in Evaluator::order_moves(board, available_moves) {
                let updated_board = board.make_move_new(mov);
                let eval = self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing, only_captures, max_depth);

                best_eval = cmp::max(best_eval, eval);

                alpha = cmp::max(alpha, best_eval);
                if beta <= alpha {
                    self.hash_put(board.get_hash(), depth, HASH_ALPHA, best_eval);
                    return alpha;
                }
            }

            self.hash_put(board.get_hash(), depth, HASH_EXACT, best_eval);
            return best_eval;
        } else {
            let mut best_eval = i32::MAX;
            for mov in Evaluator::order_moves(board, available_moves) {
                let updated_board = board.make_move_new(mov);
                let eval = self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing, only_captures, max_depth);

                best_eval = cmp::min(best_eval, eval);
                beta = cmp::min(beta, best_eval);

                if beta <= alpha {
                    self.hash_put(board.get_hash(), depth, HASH_BETA, best_eval);
                    return beta;
                }
            }

            self.hash_put(board.get_hash(), depth, HASH_EXACT, best_eval);
            return best_eval;
        }
    }

    fn total_eval(&self, board:Board) -> i32 {
        if board.status() == BoardStatus::Stalemate {
            return 0;
        }

        let piece_eval = Evaluator::piece_evaluation(board);
        let king_eval = Evaluator::king_evaluation(board);
        let move_eval = Evaluator::move_evaluation(board);
        let center_eval = Evaluator::center_evaluation(board);

        return piece_eval + move_eval + king_eval + center_eval;
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

    fn center_evaluation(board:Board) -> i32 {
        let mut white_score = 0;
        let mut black_score = 0;

        for square in [Square::E4, Square::E5, Square::D4, Square::D5] {
            match board.color_on(square) {
                Some(Color::White) => white_score += 1000,
                Some(Color::Black) => black_score += 1000,
                None => ()
            }
        }

        for square in [Square::E3, Square::E6, Square::D3, Square::D6, Square::C4, Square::C5] {
            match board.color_on(square) {
                Some(Color::White) => white_score += 500,
                Some(Color::Black) => black_score += 500,
                None => ()
            }
        }

        return -(white_score - black_score) / 16;
    }
}
