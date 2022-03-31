use chess::{Board, ChessMove, MoveGen, Color, Square, Piece};
use std::str::FromStr;

fn main() {
    let position = Board::default().to_string();
    let best_move = get_move(&position);
    println!("Best move: {}", best_move);
}

fn get_move(position:&str) -> ChessMove {
    let board = Board::from_str(&position).unwrap();
    return get_best_move(board);
}

fn get_best_move(board:Board) -> ChessMove {
    let mut moves = MoveGen::new_legal(&board);
    let side_to_move = board.side_to_move();

    let mut best_move = moves.next().unwrap();
    let mut best_eval = total_eval(board.make_move_new(best_move));

    for available_move in moves {
        let new_board = board.make_move_new(available_move);
        let eval = total_eval(new_board);

        println!("{}: {}", available_move, eval);

        if side_to_move == Color::White && eval >= best_eval {
            best_move = available_move;
            best_eval = eval;
        } else if side_to_move == Color::Black && eval <= best_eval {
            best_move = available_move;
            best_eval = eval;
        }
    }

    return best_move;
}

fn total_eval(board:Board) -> i8 {
    let piece_eval = piece_evaluation(board);

    let white_moves;
    let black_moves;

    if board.side_to_move() == Color::White {
        white_moves = available_moves(board);
        black_moves = available_moves(board.null_move().unwrap());
    } else {
        white_moves = available_moves(board.null_move().unwrap());
        black_moves = available_moves(board);
    }
    let move_eval = (white_moves - black_moves) / 10;

    let king_eval = king_evaluation(board);

    println!("{} {} {}", piece_eval, move_eval, king_eval);

    return piece_eval + move_eval + king_eval;
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

fn available_moves(board:Board) -> i8 {
    return MoveGen::new_legal(&board).len() as i8;
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
