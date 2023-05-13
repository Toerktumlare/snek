use std::{thread, time::Duration};

use super::{
    component::{
        Apple, Arena, Collidable, Debugging, Position, Render, Snek, Type, Types, Velocity,
    },
    ecs::{simulation::Simulation, system::System},
    gui::{event_handler::Action, screen::Screen, window::Window, Pos, Size},
    system::{
        AppleSpawningSystem, CollisionCheckSystem, DeathSystem, DebugSystem, MoveSystem,
        RenderSystem, VelocitySystem, WrappingBoundrySystem,
    },
};

pub struct Game {
    simulation: Simulation,
    is_running: bool,
}

impl Game {
    pub fn new(screen: Screen, arena_height: i16, arena_width: i16) -> Self {
        let window = Window::new(Pos::new(10, 1), Size::new(140, 40));
        let mut simulation = Simulation::new();
        simulation.register_component::<Position>();
        simulation.register_component::<Velocity>();
        simulation.register_component::<Render>();
        simulation.register_component::<Collidable>();
        simulation.register_component::<Arena>();
        simulation.register_component::<Snek>();
        simulation.register_component::<Apple>();
        simulation.register_component::<Debugging>();
        simulation.register_component::<Type>();

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Snek { is_alive: true });
        simulation.add_component_to_entity(entity_id, Type { typ: Types::Snek });
        simulation.add_component_to_entity(entity_id, Position { x: 7, y: 7 });
        simulation.add_component_to_entity(entity_id, Velocity { x: 1, y: 0 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🟢' });
        simulation.add_component_to_entity(entity_id, Collidable { collided: false });
        simulation.add_component_to_entity(entity_id, Debugging::default());

        let entity_id = simulation.create_entity();
        simulation.add_component_to_entity(entity_id, Apple { is_alive: true });
        simulation.add_component_to_entity(entity_id, Type { typ: Types::Apple });
        simulation.add_component_to_entity(entity_id, Position { x: 5, y: 5 });
        simulation.add_component_to_entity(entity_id, Render { sprite: '🍎' });
        simulation.add_component_to_entity(entity_id, Collidable { collided: false });
        simulation.add_component_to_entity(entity_id, Debugging::default());

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
        simulation.add_system(AppleSpawningSystem {});
        simulation.add_system(DebugSystem {});
        simulation.add_system(RenderSystem::new(window, screen));

        Self {
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
            if action == Action::Exit {
                break;
            }
        }
    }
}
