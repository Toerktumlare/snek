use std::fmt::Display;

#[derive(Debug)]
pub struct Snake {
    pub x: i16,
    pub y: i16,
    pub x_speed: i16,
    pub y_speed: i16,
}

impl Snake {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x,
            y,
            x_speed: 0,
            y_speed: 1,
        }
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, x_speed: {}, y_speed: {}",
            self.x, self.y, self.x_speed, self.y_speed
        )
    }
}
