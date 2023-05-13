use super::ecs::Component;

pub enum Types {
    Snek,
    Apple,
}

pub struct Snek {
    pub is_alive: bool,
}

pub struct Apple {
    pub is_alive: bool,
}

pub struct Position {
    pub x: i16,
    pub y: i16,
}

pub struct Velocity {
    pub x: i16,
    pub y: i16,
}

pub struct Render {
    pub sprite: char,
}

pub struct Collidable {
    pub collided: bool,
}

pub struct Arena {
    pub width: i16,
    pub height: i16,
}

pub struct Type {
    pub typ: Types,
}

#[derive(Default)]
pub struct Debugging {
    pub name: Option<String>,
    pub x: Option<i16>,
    pub y: Option<i16>,
    pub is_alive: Option<bool>,
    pub collided: Option<bool>,
    sprite: Option<char>,
    width: Option<i16>,
    height: Option<i16>,
}

impl Component for Position {}
impl Component for Velocity {}
impl Component for Render {}
impl Component for Collidable {}
impl Component for Arena {}
impl Component for Snek {}
impl Component for Apple {}
impl Component for Debugging {}
impl Component for Type {}
