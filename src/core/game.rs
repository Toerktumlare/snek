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

struct Collidable {
    collided: bool,
}

struct Arena {
    width: i16,
    height: i16,
}

impl Component for Namable {}
impl Component for Position {}
impl Component for Velocity {}
impl Component for Render {}
impl Component for Collidable {}
impl Component for Arena {}

struct MoveSystem;
struct CollisionCheckSystem;
struct WrappingBoundrySystem;

impl System for WrappingBoundrySystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor) {
        let (arena_width, arena_height) = {
            let arena = &em.borrow_components::<Arena>().unwrap()[0];
            (arena.width, arena.height)
        };
        let snek_id = eia.borrow_ids_for_pair::<Velocity, Position>(em).unwrap()[0];
        let mut snek = em.borrow_component_mut::<Position>(snek_id).unwrap();
        if snek.x == arena_height {
            snek.x = 0;
        }

        if snek.x < 0 {
            snek.x = arena_height;
        }

        if snek.y == arena_width {
            snek.y = 0;
        }

        if snek.y < 0 {
            snek.y = arena_height;
        }
    }
}

impl System for CollisionCheckSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor) {}
}

impl CollisionCheckSystem {
    fn check_collision(
        &self,
        manager: &EntityManager,
        entity_id1: usize,
        entity_id2: usize,
    ) -> bool {
        let position1 = manager.borrow_component::<Position>(entity_id1).unwrap();
        let position2 = manager.borrow_component::<Position>(entity_id2).unwrap();
        position1.x > position2.x && position1.y > position2.y
    }
}

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
    pub fn new(screen: Screen, arena_height: i16, arena_width: i16) -> Self {
        let mut simulation = Simulation::new();
        simulation.register_component::<Namable>();
        simulation.register_component::<Position>();
        simulation.register_component::<Velocity>();
        simulation.register_component::<Render>();
        simulation.register_component::<Collidable>();
        simulation.register_component::<Arena>();

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Namable { name: "snek" });
        simulation.add_component_to_entity(entity_id, Position { x: 7, y: 7 });
        simulation.add_component_to_entity(entity_id, Velocity { x: 1, y: 0 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🟢' });
        simulation.add_component_to_entity(entity_id, Collidable { collided: false });

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Namable { name: "apple" });
        simulation.add_component_to_entity(entity_id, Position { x: 5, y: 5 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🍎' });
        simulation.add_component_to_entity(entity_id, Collidable { collided: false });

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(
            entity_id,
            Arena {
                width: arena_width,
                height: arena_height,
            },
        );

        for x in 0..arena_height {
            for y in 0..arena_width {
                if x == 0 || x == arena_height - 1 || (y == 0 || y == arena_width - 1) {
                    let entity_id = simulation.create_entity();
                    simulation.add_component_to_entity(entity_id, Position { x, y });
                    simulation.add_component_to_entity(entity_id, Render { sprite: '▩' });
                    simulation.add_component_to_entity(entity_id, Collidable { collided: false });
                }
            }
        }

        simulation.add_system(MoveSystem {});
        simulation.add_system(CollisionCheckSystem {});
        simulation.add_system(WrappingBoundrySystem {});

        let window = Window::new(Pos::new(10, 1), Size::new(140, 40));

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
            self.simulation.update();
            // self.draw();

            let em = &self.simulation.entity_manager;
            let eim = &mut self.simulation.entity_id_accessor;

            let entity_ids = eim.borrow_ids_for_pair::<Render, Position>(em).unwrap();

            self.screen
                .erase_region(Pos::new(10, 1), Size::new(140, 40));
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
