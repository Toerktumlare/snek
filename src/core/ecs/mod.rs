use core::any::Any;
use std::collections::HashMap;
pub mod entities;
pub mod entityidaccessor;
pub mod entitymanager;
pub mod simulation;
pub mod system;

pub trait Component {}

pub trait ComponentManagerTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + Component> ComponentManagerTrait for ComponentManager<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

#[derive(Default)]
pub struct ComponentManager<T> {
    components: Vec<T>,
    entity_ids: Vec<usize>,
    entity_ids_map: HashMap<usize, usize>,
}

impl<T: Component> ComponentManager<T> {
    pub fn new() -> Self {
        Self {
            components: vec![],
            entity_ids: vec![],
            entity_ids_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity_id: usize, component: T) {
        self.components.push(component);
        self.entity_ids.push(entity_id);
        let index = self.entity_ids.len() - 1;
        self.entity_ids_map.insert(entity_id, index);
    }

    pub fn borrow_entity_ids(&self) -> &Vec<usize> {
        &self.entity_ids
    }

    fn has(&self, entity_id: usize) -> bool {
        self.entity_ids_map.contains_key(&entity_id)
    }

    fn borrow_components_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        let index = self.entity_ids_map.get(&entity_id).unwrap();
        Some(&mut self.components[*index])
    }
}
