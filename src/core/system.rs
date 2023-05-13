use rand::Rng;

use super::{
    component::{
        Apple, Arena, Collidable, Debugging, Position, Render, Snek, Type, Types, Velocity,
    },
    ecs::{entityidaccessor::EntityIdAccessor, entitymanager::EntityManager, system::System},
    gui::{buffer::Style, event_handler::Action, screen::Screen, window::Window, Pos, Size},
};

pub struct MoveSystem;
pub struct CollisionCheckSystem;
pub struct WrappingBoundrySystem;
pub struct VelocitySystem;
pub struct DeathSystem;
pub struct AppleSpawningSystem;
pub struct DebugSystem;
pub struct RenderSystem {
    window: Window,
    screen: Screen,
}

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
        for apple_id in apple_ids.iter() {
            if CollisionCheckSystem::check_collision(self, em, user_id, *apple_id) {
                let (c, _) = em
                    .borrow_component_pair_mut::<Collidable, Apple>(*apple_id)
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
        position1.x == position2.x && position1.y == position2.y
    }
}

impl System for MoveSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
        let entity_ids = eia.borrow_ids_for_pair::<Velocity, Position>(em).unwrap();
        for id in entity_ids.iter() {
            let (velocity, mut position) = em
                .borrow_component_pair_mut::<Velocity, Position>(*id)
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
            .borrow_component_pair_mut::<Velocity, Position>(id)
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
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
        let ids = eia.borrow_ids::<Collidable>(em).unwrap();
        for id in ids.iter() {
            let component = em.borrow_component::<Collidable>(*id).unwrap();
            if component.collided {
                let (apple, collidable) = em
                    .borrow_component_pair_mut::<Apple, Collidable>(*id)
                    .unwrap();
                collidable.collided = false;
                apple.is_alive = false;
            }
        }
    }
}

impl System for AppleSpawningSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
        let apple_id = eia.borrow_ids::<Apple>(em).unwrap()[0];
        let arena_id = eia.borrow_ids::<Arena>(em).unwrap()[0];
        let arena = em.borrow_component::<Arena>(arena_id).unwrap();
        let (apple, position) = em
            .borrow_component_pair_mut::<Apple, Position>(apple_id)
            .unwrap();

        if !apple.is_alive {
            apple.is_alive = true;
            let mut rng = rand::thread_rng();
            position.x = rng.gen_range(1..arena.width - 1);
            position.y = rng.gen_range(1..arena.height - 1);
        }
    }
}

impl DebugSystem {
    fn extract_snek_info(&self, id: usize, debug: &mut Debugging, em: &EntityManager) {
        let (position, snek, collidable) = em
            .borrow_component_triple_mut::<Position, Snek, Collidable>(id)
            .unwrap();
        {
            debug.x = Some(position.x);
            debug.y = Some(position.y);
            debug.name = Some("Snek".to_string());
            debug.is_alive = Some(snek.is_alive);
            debug.collided = Some(collidable.collided);
        }
    }

    fn extract_apple_info(&self, id: usize, debug: &mut Debugging, em: &EntityManager) {
        let (position, apple, collidable) = em
            .borrow_component_triple_mut::<Position, Apple, Collidable>(id)
            .unwrap();
        {
            debug.x = Some(position.x);
            debug.y = Some(position.y);
            debug.name = Some("Apple".to_string());
            debug.is_alive = Some(apple.is_alive);
            debug.collided = Some(collidable.collided);
        }
    }
}

impl System for DebugSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, _action: &Action) {
        let ids = eia.borrow_ids::<Type>(em).unwrap();

        for id in ids {
            let (typ, debugging) = em
                .borrow_component_pair_mut::<Type, Debugging>(*id)
                .unwrap();
            match typ.typ {
                Types::Snek => self.extract_snek_info(*id, debugging, em),
                Types::Apple => self.extract_apple_info(*id, debugging, em),
            }
        }
    }
}

impl RenderSystem {
    pub fn new(window: Window, screen: Screen) -> Self {
        Self { window, screen }
    }

    fn snake_stats(&mut self, debug: &Debugging) {
        self.window.print(
            &mut self.screen,
            format!(
                "name: {}, position.x: {}, position.y: {}, is_alive: {}, collided: {}",
                debug.name.to_owned().unwrap(),
                debug.x.unwrap(),
                debug.y.unwrap(),
                debug.is_alive.unwrap(),
                debug.collided.unwrap()
            ),
            &mut Pos::new(0, 23),
            Style::white(),
        );
    }

    fn apple_status(&mut self, debug: &Debugging) {
        self.window.print(
            &mut self.screen,
            format!(
                "name: {}, position.x: {}, position.y: {}, is_alive: {}, collided: {}",
                debug.name.to_owned().unwrap(),
                debug.x.unwrap(),
                debug.y.unwrap(),
                debug.is_alive.unwrap(),
                debug.collided.unwrap()
            ),
            &mut Pos::new(0, 24),
            Style::white(),
        );
    }
}

impl System for RenderSystem {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, action: &Action) {
        let entity_ids = eia.borrow_ids_for_pair::<Render, Position>(em).unwrap();
        self.screen
            .erase_region(Pos::new(10, 1), Size::new(140, 40));
        for id in entity_ids.iter() {
            let render = em.borrow_component::<Render>(*id).unwrap();
            let position = em.borrow_component::<Position>(*id).unwrap();
            let debug = em.borrow_component::<Debugging>(*id);

            self.window.put_sprite(
                &mut self.screen,
                render.sprite,
                Pos::new(position.x as u16, position.y as u16),
                Style::white(),
            );

            if let Some(debug) = debug {
                match &debug.name {
                    Some(name) => {
                        if name == "Snek" {
                            self.snake_stats(debug);
                        } else if name == "Apple" {
                            self.apple_status(debug);
                        }
                    }
                    None => todo!(),
                }
            }
        }
        self.screen.render().unwrap();
        if action == &Action::Exit {
            self.screen.disable_raw_mode().unwrap();
        }
    }
}
