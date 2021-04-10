#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_give_opposite_of_up() {
        let d = Direction::Up.opposite();
        assert_eq!(d, Direction::Down);
    }

    #[test]
    fn should_give_opposite_of_right() {
        let d = Direction::Right.opposite();
        assert_eq!(d, Direction::Left);
    }

    #[test]
    fn should_give_opposite_of_down() {
        let d = Direction::Down.opposite();
        assert_eq!(d, Direction::Up);
    }

    #[test]
    fn should_give_opposite_of_left() {
        let d = Direction::Left.opposite();
        assert_eq!(d, Direction::Right);
    }
}
