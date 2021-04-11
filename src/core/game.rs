use crossterm::{
    cursor::Hide,
    style::{Color, SetForegroundColor},
    terminal::{enable_raw_mode, size, Clear, ClearType, SetSize},
    ExecutableCommand,
};
use std::io::Stdout;
use std::time::Duration;
use std::time::Instant;

const MAX_SPEED: u16 = 20;
const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;

#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    speed: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size = size().unwrap();
        Self {
            stdout,
            original_terminal_size,
            width,
            height,
            speed: 0,
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.render();

        let mut done = false;

        while !done {
            let interval = self.calculate_interval();
            let now = Instant::now();
            while now.elapsed() < interval {
                println!("Tick");
                println!(
                    "interval: {}, elapsed: {}",
                    interval.as_millis(),
                    now.elapsed().as_millis()
                );
            }
        }
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64,
        )
    }

    fn init(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    fn render(&mut self) {
        self.draw_borders();
    }

    fn draw_borders(&mut self) {
        self.stdout
            .execute(SetForegroundColor(Color::DarkGrey))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::stdout;

    #[test]
    fn should_return_max_interval_at_lowest_speed() {
        let game = Game::new(stdout(), 10, 10);
        let duration = game.calculate_interval();
        assert_eq!(duration.as_millis(), 700);
    }
}
