use crate::core::gui::event_handler::{Action, EventHandler};

use super::{
    entityidaccessor::EntityIdAccessor, entitymanager::EntityManager, system::System, Component,
};

#[derive(Default)]
pub struct Simulation {
    pub entity_manager: EntityManager,
    pub entity_id_accessor: EntityIdAccessor,
    pub event_handler: EventHandler,
    systems: Vec<Box<dyn System>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            entity_id_accessor: EntityIdAccessor::new(),
            event_handler: EventHandler::new(),
            systems: vec![],
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.entity_manager.remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut Self {
        self.entity_manager.register::<T>();
        self
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        self.entity_manager
            .add_component_to_entity(entity_id, component);
        self
    }

    pub fn update(&mut self) -> Action {
        let event = if let Ok(event) = self.event_handler.receiver.try_recv() {
            event
        } else {
            Action::None
        };

        for system in self.systems.iter_mut() {
            system.update(
                &mut self.entity_manager,
                &mut self.entity_id_accessor,
                &event,
            );
            self.entity_manager.step_frame();
        }

        event
    }
}
