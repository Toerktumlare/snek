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

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn slither(&mut self) {
        self.body
            .insert(0, self.get_head().transform(self.get_direction(), 1));
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        } else {
            self.digesting = false;
        }
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

    #[test]
    fn should_contain_point() {
        let start = Point::new(5, 6);
        let snake = Snake::new(start, 3, Direction::Right);
        let other_point = Point::new(3, 6);
        assert!(snake.contains_point(&other_point));
    }

    #[test]
    fn should_not_contain_point() {
        let start = Point::new(5, 6);
        let snake = Snake::new(start, 3, Direction::Right);
        let other_point = Point::new(5, 7);
        assert!(!snake.contains_point(&other_point));
    }

    #[test]
    fn slither_one_step_if_digesting_keep_tail() {
        let start = Point::new(5, 5);
        let mut snake = Snake::new(start, 3, Direction::Right);
        snake.grow();
        snake.slither();
        assert_eq!(snake.get_head(), Point::new(6, 5));

        let tail = Point::new(3, 5);
        assert!(snake.contains_point(&tail));
    }

    #[test]
    fn slither_one_step_if_not_digesting_remove_tail() {
        let start = Point::new(5, 5);
        let mut snake = Snake::new(start, 3, Direction::Right);
        snake.slither();
        assert_eq!(snake.get_head(), Point::new(6, 5));

        let tail = Point::new(3, 5);
        assert!(!snake.contains_point(&tail));
    }
}
