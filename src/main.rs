use chess::*;
use std::str::FromStr;

mod evaluator;
mod uci;

use crate::evaluator::Evaluator;

fn main() {
    let evaluator = Evaluator::new();
    uci::parse_with(evaluator);
}
