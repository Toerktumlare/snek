pub mod core;

use crate::core::game::Game;
use std::io::stdout;

fn main() {
    println!("Starting game...");
    Game::new(stdout(), 50, 25).run();
}
