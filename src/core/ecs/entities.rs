#[derive(Debug)]
struct Entity {
    alive: bool,
}

impl Entity {
    pub fn new() -> Entity {
        Self { alive: true }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn resurrect(&mut self) {
        self.alive = true;
    }
}

#[derive(Default)]
pub struct Entities {
    entities: Vec<Entity>,
    available: Vec<usize>,
}

/// Struct that holds entities
///
/// Each entity can be alive or dead. When an entity is killed off it does not get removed it gets
/// marked as killed and it\s index index gets pushed into a pool of avaliable entities that gets
/// resurrected when a new entity is spawned. This will make sure that we dont do a lot of
/// unneccasary allocations of new entities when they are killed/removed.
///
/// TODO: maybe look into some sort of garbage collection system that will systematically remove
/// killed off entities that dont get respawned in a certain amount of time/frames.
impl Entities {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            available: vec![],
        }
    }

    fn push(&self, _entity: Entity) -> usize {
        todo!()
    }

    pub fn create(&mut self) -> usize {
        if self.available.len() > 0 {
            let index = self.available.remove(0);
            self.entities[index].resurrect();
            return index;
        }
        let entity = Entity::new();
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn remove(&mut self, entity_id: usize) {
        if !self.has(entity_id) {
            return;
        }
        self.entities[entity_id].kill();
        self.available.push(entity_id);
    }

    pub fn has(&self, entity_id: usize) -> bool {
        entity_id < self.entities.len() && self.entities[entity_id].is_alive()
    }
}
