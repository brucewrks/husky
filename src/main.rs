// use std::cmp;
use std::str::FromStr;

use chess::{Board, ChessMove, MoveGen, Color, Square, Piece, BoardStatus};

fn main() {
    // let position = Board::default().to_string();
    let position = "r5k1/6p1/4N1Pp/p4P1n/2K5/1R6/P7/8 w - - 0 58";

    let best_move = best_move(&position);
    println!("Best move: {}", best_move);
}

fn best_move(position:&str) -> ChessMove {
    let board = Board::from_str(&position).unwrap();
    let mv;
    let _eval;

    if board.side_to_move() == Color::White {
        (mv, _eval) = max_ab(board, 125, -125, 0);
    } else {
        (mv, _eval) = min_ab(board, -125, 125, 0);
    }

    return mv;
}

fn empty_move() -> ChessMove {
    return ChessMove::new(Square::A1, Square::A2, None);
}

fn max_ab(board:Board, alpha:i8, beta:i8, depth:u8) -> (ChessMove, i8) {
    if depth >= 8 {
        return (empty_move(), total_eval(board));
    }

    let mut possible_moves = MoveGen::new_legal(&board);

    let mut beta = beta;

    let mut top_move = empty_move();
    let mut top_board;
    let mut top_score = -120;

    for (index, possible_move) in possible_moves.enumerate() {
        let new_board = board.make_move_new(possible_move);
        let (the_move, score) = max_ab(new_board, alpha, beta, depth + 1);

        if score > top_score || top_move == empty_move() {
            top_move = the_move;
            top_board = index;
            top_score = score;
        }

        if score > alpha {
            return (the_move, top_score);
        }
        if score > beta {
            beta = score;
        }
    }

    return (top_move, top_score);
}

fn min_ab(board:Board, alpha:i8, beta:i8, depth:u8) -> (ChessMove, i8) {
    if depth >= 8 {
        return (empty_move(), total_eval(board));
    }

    let mut possible_moves = MoveGen::new_legal(&board);

    let mut beta = beta;
    let mut lowest_move = empty_move();

    let mut lowest_board;
    let mut lowest_score = 120;

    for (index, possible_move) in possible_moves.enumerate() {
        let new_board = board.make_move_new(possible_move);
        let (the_move, score) = max_ab(new_board, alpha, beta, depth + 1);

        if score < lowest_score || lowest_move == empty_move() {
            lowest_move = the_move;
            lowest_board = index;
            lowest_score = score;
        }

        if score < alpha {
            return (the_move, lowest_score);
        }
        if score < beta {
            beta = score;
        }
    }

    return (lowest_move, lowest_score);
}

fn total_eval(board:Board) -> i8 {
    if board.status() == BoardStatus::Stalemate {
        return 0;
    }
    if board.status() == BoardStatus::Checkmate {
        if board.side_to_move() == Color::White {
            return -120;
        }
        return 120;
    }

    let piece_eval = piece_evaluation(board);
    let king_eval = king_evaluation(board);
    let move_eval = move_evaluation(board);

    println!("{} {} {}", piece_eval, king_eval, move_eval);

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
