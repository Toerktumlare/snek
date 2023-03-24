pub mod core;

use std::env::args;

use crate::core::game::Game;

fn main() {
    let args: Vec<String> = args().collect();
    let height = &args[1];
    let width = &args[2];

    let screen = core::gui::screen::Screen::stdout()
        .unwrap()
        .alternate_screen(false);
    Game::new(screen, height.parse().unwrap(), width.parse().unwrap()).run();
}
