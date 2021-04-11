pub mod core;

use crate::core::game::Game;
use std::io::stdout;

fn main() {
    println!("Starting game...");
    Game::new(stdout(), 10, 10).run();
}
