use std::io;

pub fn parse_with(evaluator:crate::evaluator::Evaluator) {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        line.clear();
        stdin.read_line(&mut line).unwrap();

        if line.is_empty() {
            break;
        }

        let argv: Vec<&str> = line.split_whitespace().collect();
        let cmd = argv[0].trim();

        match cmd {
            "uci" => (),
            "setoption" => (),
            "ucinewgame" => (),
            "isready" => (),
            "position" => (),
            "go" => (),
            "quit" => break,
            _ => println!("Unrecognized command: {}", cmd),
        }
    }
}
