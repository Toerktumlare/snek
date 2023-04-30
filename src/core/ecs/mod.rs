use core::any::Any;
use std::{any::TypeId, collections::HashMap};
pub mod entities;
pub mod entityidaccessor;
pub mod entitymanager;
pub mod simulation;
pub mod system;

pub trait Component {}

pub trait ComponentManagerTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn has(&self, entity_id: usize) -> bool;
    fn remove(&mut self, entity_id: usize);
    fn get_type_id(&self) -> TypeId;
}

impl<T: 'static + Component> ComponentManagerTrait for ComponentManager<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn has(&self, entity_id: usize) -> bool {
        let manager = cast_manager::<T>(self);
        manager.has(entity_id)
    }

    fn remove(&mut self, entity_id: usize) {
        let manager = cast_manager_mut::<T>(self);
        manager.remove(entity_id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

pub fn cast_manager<T: 'static + Component>(
    manager: &dyn ComponentManagerTrait,
) -> &ComponentManager<T> {
    manager
        .as_any()
        .downcast_ref::<ComponentManager<T>>()
        .unwrap()
}

pub fn cast_manager_mut<T: 'static + Component>(
    manager: &mut dyn ComponentManagerTrait,
) -> &mut ComponentManager<T> {
    manager
        .as_any_mut()
        .downcast_mut::<ComponentManager<T>>()
        .unwrap()
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

    pub fn remove(&mut self, entity_id: usize) {
        if !self.has(entity_id) {
            return;
        }

        let index = *self.entity_ids_map.get(&entity_id).unwrap();
        self.entity_ids_map
            .insert(*self.entity_ids.last().unwrap(), index);
        self.components.swap_remove(index);
        self.entity_ids.swap_remove(index);
        self.entity_ids_map.remove(&entity_id);
    }

    fn borrow_components_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        if !self.has(entity_id) {
            return None;
        }
        let index = self.entity_ids_map.get(&entity_id).unwrap();
        Some(&mut self.components[*index])
    }

    fn borrow_component(&self, entity_id: usize) -> Option<&T> {
        if !self.has(entity_id) {
            return None;
        }
        let index = self.entity_ids_map.get(&entity_id).unwrap();
        Some(&self.components[*index])
    }

    fn borrow_component_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        let index = self.entity_ids_map.get(&entity_id).unwrap();
        Some(&mut self.components[*index])
    }

    fn borrow_components(&self) -> Option<&Vec<T>> {
        Some(&self.components)
    }
}
