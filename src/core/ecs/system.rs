use super::{entityidaccessor::EntityIdAccessor, entitymanager::EntityManager};

pub trait System {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor);
}
