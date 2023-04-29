use std::io::Write;

use unicode_width::UnicodeWidthChar;

use super::{
    buffer::{Cell, Style},
    screen::Screen,
    Pos, Size,
};

pub struct Window {
    absolute_pos: Pos,
    pub size: Size,
    pub cursor: Pos,
}

impl Window {
    pub fn new(pos: Pos, size: Size) -> Self {
        Self {
            absolute_pos: pos,
            size,
            cursor: Pos::zero(),
        }
    }

    pub fn print(&mut self, screen: &mut Screen, s: impl AsRef<str>, pos: &mut Pos, style: Style) {
        for c in s.as_ref().chars() {
            pos.x += 1;
            self.put(screen, c, *pos, style)
        }
    }

    pub fn put(&mut self, screen: &mut Screen, c: char, pos: Pos, style: Style) {
        let pos = Pos::new(pos.x * 2, pos.y);
        let c_width = c.width().unwrap();
        if c_width > 1 {
            let cell = Cell::new(c, style);
            screen.put(cell, self.absolute_pos + self.cursor + pos);
            for i in 1..c_width {
                screen.put(
                    Cell::continuation(style),
                    self.absolute_pos + self.cursor + pos + Pos::new(pos.x + i as u16, pos.y),
                );
            }
        } else {
            let cell = Cell::new(c, style);
            screen.put(cell, self.absolute_pos + self.cursor + pos);
        }
    }

    pub fn clear(&mut self, screen: &mut Screen) {
        screen.erase_region(self.absolute_pos, self.size);
        self.cursor = Pos::zero();
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn should_place_char_at_zero_zero() {}
}
