pub mod core;

use crate::core::game::Game;

fn main() {
    let screen = core::gui::screen::Screen::stdout()
        .unwrap()
        .alternate_screen(true);
    Game::new(screen, 50, 25).run();
}
