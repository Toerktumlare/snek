use super::direction::Direction;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
        };

        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!(
                "Transforming value {} by {} would result in a negative value",
                value, by
            );
        }
        (value as i16 + by) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_transform_positive_value() {
        let result = Point::transform_value(1, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn should_not_panic_if_result_is_zero() {
        let result = Point::transform_value(1, -1);
        assert_eq!(result, 0);
    }

    #[test]
    fn should_not_panic_if_result_is_positive() {
        let result = Point::transform_value(2, -1);
        assert_eq!(result, 1);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_result_is_negative() {
        Point::transform_value(2, -3);
    }
}
