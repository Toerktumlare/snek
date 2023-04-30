use std::{thread, time::Duration};

use super::{
    ecs::{
        entityidaccessor::EntityIdAccessor, entitymanager::EntityManager, simulation::Simulation,
        system::System, Component,
    },
    gui::{buffer::Style, event_handler::Action, screen::Screen, window::Window, Pos, Size},
};

pub struct Game {
    screen: Screen,
    window: Window,
    simulation: Simulation,
    is_running: bool,
}

pub struct Snek {
    pub is_alive: bool,
}

pub struct Apple {
    pub is_alive: bool,
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

impl Component for Position {}
impl Component for Velocity {}
impl Component for Render {}
impl Component for Collidable {}
impl Component for Arena {}
impl Component for Snek {}
impl Component for Apple {}

struct MoveSystem;
struct CollisionCheckSystem;
struct WrappingBoundrySystem;
struct VelocitySystem;
struct DeathSystem;

impl System for WrappingBoundrySystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
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
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
        let user_id = eia.borrow_ids_for_pair::<Position, Snek>(em).unwrap()[0];
        let apple_ids = eia.borrow_ids_for_pair::<Position, Apple>(em).unwrap();
        for id in apple_ids.iter() {
            if CollisionCheckSystem::check_collision(self, em, user_id, *id) {
                let (c, _) = em
                    .borrow_component_manager_pair_mut::<Collidable, Apple>(*id)
                    .unwrap();
                c.collided = true;
            }
        }
    }
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
        position1.x <= position2.x && position1.y <= position2.y
    }
}

impl System for MoveSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
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

impl System for VelocitySystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, action: &Action) {
        let id = eia.borrow_ids_for_pair::<Velocity, Position>(em).unwrap()[0];
        let (velocity, _) = em
            .borrow_component_manager_pair_mut::<Velocity, Position>(id)
            .unwrap();
        match action {
            Action::Up => {
                velocity.x = 0;
                velocity.y = -1;
            }
            Action::Down => {
                velocity.x = 0;
                velocity.y = 1;
            }
            Action::Left => {
                velocity.x = -1;
                velocity.y = 0;
            }
            Action::Right => {
                velocity.x = 1;
                velocity.y = 0;
            }
            _ => (),
        }
    }
}

impl System for DeathSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, action: &Action) {
        let ids = eia.borrow_ids::<Collidable>(em).unwrap();
        for id in ids.iter() {
            let component = em.borrow_component::<Collidable>(*id).unwrap();
            if component.collided {
                let apple = em.borrow_component_mut::<Apple>(*id).unwrap();
                apple.is_alive = false;
                em.remove_entity(*id);
            }
        }
    }
}

impl Game {
    pub fn new(screen: Screen, arena_height: i16, arena_width: i16) -> Self {
        let mut simulation = Simulation::new();
        simulation.register_component::<Position>();
        simulation.register_component::<Velocity>();
        simulation.register_component::<Render>();
        simulation.register_component::<Collidable>();
        simulation.register_component::<Arena>();
        simulation.register_component::<Snek>();
        simulation.register_component::<Apple>();

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Snek { is_alive: true });
        simulation.add_component_to_entity(entity_id, Position { x: 7, y: 7 });
        simulation.add_component_to_entity(entity_id, Velocity { x: 1, y: 0 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🟢' });
        simulation.add_component_to_entity(entity_id, Collidable { collided: false });

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Apple { is_alive: true });
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

        simulation.add_system(VelocitySystem {});
        simulation.add_system(MoveSystem {});
        simulation.add_system(CollisionCheckSystem {});
        simulation.add_system(WrappingBoundrySystem {});
        simulation.add_system(DeathSystem {});

        let window = Window::new(Pos::new(10, 1), Size::new(140, 40));

        screen.enable_raw_mode().unwrap();

        Self {
            screen,
            window,
            simulation,
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;

        let duration = Duration::from_millis(1000 / 15);
        while self.is_running {
            thread::sleep(duration);
            let action = self.simulation.update();

            let em = &self.simulation.entity_manager;
            let eim = &mut self.simulation.entity_id_accessor;

            let entity_ids = eim.borrow_ids_for_pair::<Render, Position>(em).unwrap();

            self.screen
                .erase_region(Pos::new(10, 1), Size::new(140, 40));
            for id in entity_ids.iter() {
                let render = em.borrow_component::<Render>(*id).unwrap();
                let position = em.borrow_component::<Position>(*id).unwrap();

                self.window.put(
                    &mut self.screen,
                    render.sprite,
                    Pos::new(position.x as u16, position.y as u16),
                    Style::white(),
                );
            }
            self.window.print(
                &mut self.screen,
                format!("em: {:?}", em.entities.entities.len()),
                &mut Pos::new(0, 23),
                Style::white(),
            );
            self.screen.render().unwrap();
            if action == Action::Exit {
                self.screen.disable_raw_mode().unwrap();
                break;
            }
        }
    }
}
