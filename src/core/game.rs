use std::{thread, time::Duration};

use super::{
    ecs::{
        entityidaccessor::EntityIdAccessor, entitymanager::EntityManager, simulation::Simulation,
        system::System, Component,
    },
    gui::{buffer::Style, screen::Screen, window::Window, Direction, Pos, Shape, Size},
};

pub struct Game {
    screen: Screen,
    window: Window,
    simulation: Simulation,
    is_running: bool,
}

struct Namable {
    name: &'static str,
}

struct Position {
    x: i16,
    y: i16,
}

struct Velocity {
    x: i16,
    y: i16,
}

struct Render {
    sprite: char,
}

impl Component for Namable {}
impl Component for Position {}
impl Component for Velocity {}
impl Component for Render {}

struct MoveSystem;

impl System for MoveSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor) {
        let entity_ids = eia.borrow_ids_for_pair::<Velocity, Position>(em).unwrap();
        for id in entity_ids.iter() {
            let (velocity, mut position) = em
                .borrow_component_manager_pair_mut::<Velocity, Position>(*id)
                .unwrap();
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

impl Game {
    pub fn new(screen: Screen, _arena_width: i16, _arena_height: i16) -> Self {
        let mut simulation = Simulation::new();
        simulation.register_component::<Namable>();
        simulation.register_component::<Position>();
        simulation.register_component::<Velocity>();
        simulation.register_component::<Render>();

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Namable { name: "snek" });
        simulation.add_component_to_entity(entity_id, Position { x: 5, y: 5 });
        simulation.add_component_to_entity(entity_id, Velocity { x: 0, y: 1 });

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Namable { name: "apple" });
        simulation.add_component_to_entity(entity_id, Position { x: 5, y: 5 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🍎' });

        let window = Window::new(Pos::new(10, 1), Size::new(30, 30));

        Self {
            screen,
            window,
            simulation,
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.init();

        self.is_running = true;

        let duration = Duration::from_millis(1000 / 15);
        while self.is_running {
            thread::sleep(duration);
            // self.simulation.update();
            self.draw();

            let em = &self.simulation.entity_manager;
            let eim = &mut self.simulation.entity_id_accessor;

            let entity_ids = eim.borrow_ids_for_pair::<Render, Position>(em).unwrap();

            for id in entity_ids.iter() {
                let (render, position) = em
                    .borrow_component_manager_pair_mut::<Render, Position>(*id)
                    .unwrap();

                self.window.put(
                    &mut self.screen,
                    render.sprite,
                    Pos::new(position.x as u16, position.y as u16),
                    Style::white(),
                )
            }
            self.screen.render().unwrap();
        }
    }

    fn init(&mut self) {
        // self.screen.enable_raw_mode().unwrap();
        // hide cursor
        // enable raw mode
    }

    fn draw(&mut self) {
        let mut shape = Shape::new(&mut self.window);
        //Top line
        shape.line(
            &mut self.screen,
            Pos::new(0, 0),
            12,
            Direction::East,
            '▩',
            Style::white(),
        );
        // left line
        shape.line(
            &mut self.screen,
            Pos::new(0, 0),
            12,
            Direction::South,
            '▩',
            Style::white(),
        );
        // right line
        shape.line(
            &mut self.screen,
            Pos::new(11, 0),
            12,
            Direction::South,
            '▩',
            Style::white(),
        );
        //bottom line
        shape.line(
            &mut self.screen,
            Pos::new(0, 12),
            12,
            Direction::East,
            '▩',
            Style::white(),
        );
        // draw screen
    }
}
