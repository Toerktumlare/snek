use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize},
    ExecutableCommand, QueueableCommand,
};
use std::{
    convert::TryInto,
    io::{Stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use super::{direction::Direction, snake::Snake};

#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    width: i16,
    height: i16,
    snake: Snake,
    tx: Sender<KeyCode>,
    rx: Receiver<KeyCode>,
    is_running: bool,
}

impl Game {
    pub fn new(stdout: Stdout, width: i16, height: i16) -> Self {
        let snake = Snake::new(3, 3);
        let (tx, rx): (Sender<KeyCode>, Receiver<KeyCode>) = mpsc::channel();
        Self {
            stdout,
            width,
            height,
            snake,
            tx,
            rx,
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.draw();
        let duration = Duration::from_millis(1000 / 15);

        let tx = self.tx.clone();

        let handle = thread::Builder::new()
            .name("key_events".to_string())
            .spawn(move || {
                Game::key_events(tx).unwrap();
            })
            .unwrap();

        self.is_running = true;

        while self.is_running {
            thread::sleep(duration);
            let mut snake = &mut self.snake;
            snake.x += snake.x_speed;
            snake.y += snake.y_speed;
            if snake.x > self.height {
                snake.x = 1;
            }
            if snake.x == 0 {
                snake.x = self.height;
            }
            if snake.y > self.width {
                snake.y = 1;
            }
            if snake.y == 0 {
                snake.y = self.width;
            }
            self.update();
            self.draw();
        }

        handle.join().unwrap();
    }

    // fn calculate_interval(&self) -> Duration {
    //     let speed = MAX_SPEED - self.speed;
    //     Duration::from_millis(
    //         (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64,
    //     )
    // }

    fn init(&mut self) {
        self.stdout
            .queue(SetSize((self.width + 3) as u16, (self.height + 3) as u16))
            .unwrap()
            .queue(Clear(ClearType::All))
            .unwrap()
            .queue(Hide)
            .unwrap();
        enable_raw_mode().unwrap();
    }

    fn draw(&mut self) {
        self.clear();
        self.draw_borders();
        self.draw_snake();
        self.draw_diagnostics();
        self.stdout.flush().unwrap();
    }

    fn clear(&mut self) {
        self.stdout.execute(Clear(ClearType::All)).unwrap();
    }

    fn draw_borders(&mut self) {
        self.stdout
            .execute(SetForegroundColor(Color::DarkGrey))
            .unwrap();

        for y in 0..self.height + 2 {
            self.stdout
                .queue(MoveTo(0, y.try_into().unwrap()))
                .unwrap()
                .queue(Print("#"))
                .unwrap()
                .queue(MoveTo((self.width + 1) as u16, y.try_into().unwrap()))
                .unwrap()
                .queue(Print("#"))
                .unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .queue(MoveTo(x.try_into().unwrap(), 0))
                .unwrap()
                .queue(Print("#"))
                .unwrap()
                .queue(MoveTo(x.try_into().unwrap(), (self.height + 1) as u16))
                .unwrap()
                .queue(Print("#"))
                .unwrap();
        }
    }

    fn draw_snake(&mut self) {
        self.stdout
            .queue(MoveTo(self.snake.y as u16, self.snake.x as u16))
            .unwrap()
            .queue(Print("*"))
            .unwrap();
    }

    fn draw_diagnostics(&mut self) {
        let snake_diagnostics = format!("Snake: {}", self.snake);
        self.stdout
            .queue(MoveTo(0, (self.height + 3) as u16))
            .unwrap()
            .queue(Print("==== Diagnostics ===="))
            .unwrap()
            .queue(MoveTo(0, (self.height + 4) as u16))
            .unwrap()
            .queue(Print(snake_diagnostics))
            .unwrap();
        self.print_direction();
    }

    fn print_direction(&mut self) {
        let mut direction = Direction::Right;
        if self.snake.y_speed == -1 && self.snake.x_speed == 0 {
            direction = Direction::Left;
        } else if self.snake.y_speed == 1 && self.snake.x_speed == 0 {
            direction = Direction::Right;
        } else if self.snake.y_speed == 0 && self.snake.x_speed == -1 {
            direction = Direction::Up;
        } else if self.snake.y_speed == 0 && self.snake.x_speed == 1 {
            direction = Direction::Down;
        }
        let direction = format!("Direction: {:?}", direction);
        self.stdout
            .queue(MoveTo(0, (self.height + 5) as u16))
            .unwrap()
            .queue(Print(direction))
            .unwrap();
    }

    fn key_events(sender: Sender<KeyCode>) -> crossterm::Result<()> {
        let mut is_running = true;
        while is_running {
            if poll(Duration::from_millis(100))? {
                if let Event::Key(event) = read()? {
                    if event.code == KeyCode::Esc {
                        is_running = false;
                        sender.send(event.code).unwrap();
                    } else {
                        sender.send(event.code).unwrap();
                    }
                }
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        if let Ok(key_code) = self.rx.try_recv() {
            match key_code {
                KeyCode::Left => {
                    self.snake.y_speed = -1;
                    self.snake.x_speed = 0;
                }
                KeyCode::Right => {
                    self.snake.y_speed = 1;
                    self.snake.x_speed = 0;
                }
                KeyCode::Up => {
                    self.snake.y_speed = 0;
                    self.snake.x_speed = -1;
                }
                KeyCode::Down => {
                    self.snake.y_speed = 0;
                    self.snake.x_speed = 1;
                }
                KeyCode::Esc => self.is_running = false,
                _ => (),
            }
        };
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode");
        self.stdout
            .queue(Clear(ClearType::All))
            .unwrap()
            .queue(Show)
            .unwrap();
        self.stdout.flush().unwrap();
    }
}
