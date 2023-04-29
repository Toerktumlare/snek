use crate::core::gui::event_handler::Action;

use super::{entityidaccessor::EntityIdAccessor, entitymanager::EntityManager};

pub trait System {
    fn update(&mut self, em: &mut EntityManager, eia: &mut EntityIdAccessor, action: &Action);
}
