use std::cmp;
use chess::*;

pub struct Evaluator {
    max_depth: u8,
}

impl Evaluator {

    pub fn new (max_depth: u8) -> Evaluator {
        return Evaluator {
            max_depth
        }
    }

    pub fn get_eval(&self, board:Board) -> i8 {
        let is_maximizing = board.side_to_move() == Color::White;
        let result =  self.minimax(0, board, -120, 120, is_maximizing);
        return result;
    }

    fn minimax(&self, depth:u8, board:Board, alpha:i8, beta:i8, is_maximizing:bool) -> i8 {
        if depth >= self.max_depth {
            return -1 * self.total_eval(board);
        }

        let mut alpha = alpha;
        let mut beta = beta;
        let available_moves = MoveGen::new_legal(&board);

        if is_maximizing {
            let mut best_eval = -120;
            for mov in available_moves {
                let updated_board = board.make_move_new(mov);
                best_eval = cmp::max(best_eval, self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing));

                alpha = cmp::max(alpha, best_eval);
                if beta <= alpha {
                    return best_eval;
                }
            }
            return best_eval;
        } else {
            let mut best_eval = 120;
            for mov in available_moves {
                let updated_board = board.make_move_new(mov);
                best_eval = cmp::min(best_eval, self.minimax(depth + 1, updated_board, alpha, beta, !is_maximizing));

                beta = cmp::min(beta, best_eval);
                if beta <= alpha {
                    return best_eval;
                }
            }
            return best_eval;
        }
    }

    fn total_eval(&self, board:Board) -> i8 {
        if board.status() == BoardStatus::Stalemate {
            return 0;
        }
        if board.status() == BoardStatus::Checkmate {
            if board.side_to_move() == Color::White {
                return -120;
            }
            return 120;
        }

        let piece_eval = Evaluator::piece_evaluation(board);
        let king_eval = Evaluator::king_evaluation(board);
        let move_eval = Evaluator::move_evaluation(board);

        // println!("{} {} {}", piece_eval, king_eval, move_eval);

        return piece_eval + move_eval + king_eval;
    }

    fn piece_evaluation(board:Board) -> i8 {
        let mut white_score = 0;
        let mut black_score = 0;

        let white = board.color_combined(Color::White).clone();
        let black = board.color_combined(Color::Black).clone();

        for sq in white {
            match board.piece_on(sq).unwrap() {
                Piece::Pawn => white_score += 1,
                Piece::Knight => white_score += 3,
                Piece::Bishop => white_score += 3,
                Piece::Rook => white_score += 5,
                Piece::Queen => white_score += 9,
                Piece::King => ()
            }
        }

        for sq in black {
            match board.piece_on(sq).unwrap() {
                Piece::Pawn => black_score += 1,
                Piece::Knight => black_score += 3,
                Piece::Bishop => black_score += 3,
                Piece::Rook => black_score += 5,
                Piece::Queen => black_score += 9,
                Piece::King => ()
            }
        }

        return white_score - black_score;
    }

    fn king_evaluation(board:Board) -> i8 {
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
                white_available += 1;
            }
        }

        let black_squares: [Option<Square>; 4] = [black_sq.up(), black_sq.down(), black_sq.left(), black_sq.right()];
        for sq in black_squares {
            if sq == None {
                continue;
            }

            let king_move = ChessMove::new(black_sq, sq.unwrap(), None);
            if board.legal(king_move) {
                black_available += 1;
            }
        }

        return (white_available - black_available) / 16;
    }

    fn move_evaluation(board:Board) -> i8 {
        // Can't check move eval if player is in check.
        if board.checkers().to_size(0) > 0 {
            return 0;
        }

        let white_moves;
        let black_moves;

        if board.side_to_move() == Color::White {
            white_moves = MoveGen::new_legal(&board).len() as i8;
            black_moves = MoveGen::new_legal(&board.null_move().unwrap()).len() as i8;
        } else {
            white_moves = MoveGen::new_legal(&board.null_move().unwrap()).len() as i8;
            black_moves = MoveGen::new_legal(&board).len() as i8;
        }

        return (white_moves - black_moves) / 10;
    }
}
