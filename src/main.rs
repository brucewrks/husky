use chess::*;
use std::str::FromStr;

mod evaluator;
mod uci;

use crate::evaluator::Evaluator;

fn main() {
    let evaluator = Evaluator::new();
    let mut uci_engine = uci::new(evaluator);
    uci_engine.parse();
}
