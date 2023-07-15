use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod game;
mod monkey;

mod prelude {
    pub use crate::game::*;
    pub use crate::monkey::*;
}

use prelude::*;

fn main() {
    run_example("example.txt");
}

fn run_example(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let _game = Game::new(read.lines().map(std::result::Result::unwrap));
}
