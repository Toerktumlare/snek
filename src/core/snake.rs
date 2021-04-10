use super::{direction::Direction, point::Point};

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();
        Self {
            body,
            direction,
            digesting: false,
        }
    }

    pub fn get_head(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn get_body(&self) -> Vec<Point> {
        self.body.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_snake_head() {
        let start = Point::new(5, 5);
        let snake = Snake::new(start, 3, Direction::Right);
        let head = snake.get_head();

        assert_eq!(head.x, 5);
        assert_eq!(head.y, 5);
    }

    #[test]
    fn should_return_body() {
        let start = Point::new(5, 5);
        let snake = Snake::new(start, 3, Direction::Right);
        let body = snake.get_body();

        assert_eq!(body.len(), 3);
    }
}
