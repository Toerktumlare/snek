use std::{any::TypeId, collections::HashMap, mem::transmute};

use super::{entities::Entities, Component, ComponentManager, ComponentManagerTrait};

#[derive(Default)]
pub struct EntityManager {
    entities: Entities,
    manager_map: HashMap<TypeId, Box<dyn ComponentManagerTrait>>,
    frame: u64,
    last_updated_map: HashMap<TypeId, u64>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
            manager_map: HashMap::new(),
            frame: 0,
            last_updated_map: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.create()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.entities.remove(entity_id);
    }

    pub fn get_last_updated_frame<T: 'static + Component>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        *self.last_updated_map.get(&type_id).unwrap()
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        let component_manager = self.borrow_component_manager_mut::<T>();
        component_manager.add(entity_id, component);
        self
    }

    pub(crate) fn register<T: 'static + Component>(&mut self) -> &mut Self {
        // @TODO: Check if component manager for T already exists
        let type_id = TypeId::of::<T>();
        self.manager_map
            .insert(type_id, Box::new(ComponentManager::<T>::new()));
        self.last_updated_map.insert(type_id, self.frame);
        self
    }

    pub(crate) fn borrow_entity_ids<T: 'static + Component>(&self) -> Option<&Vec<usize>> {
        Some(self.borrow_component_manager::<T>().borrow_entity_ids())
    }

    fn borrow_component_manager_mut<T: 'static + Component>(&mut self) -> &mut ComponentManager<T> {
        let type_id = TypeId::of::<T>();

        // Handle if there is no manager for a component
        let unknown = self.manager_map.get_mut(&type_id).unwrap().as_mut();
        let manager = cast_manager_mut(unknown);
        manager
    }

    pub(crate) fn borrow_component_manager<T: 'static + Component>(&self) -> &ComponentManager<T> {
        let type_id = TypeId::of::<T>();

        // Handle if there is no manager for a component
        let unknown = self.manager_map.get(&type_id).unwrap().as_ref();
        let manager = cast_manager(unknown);
        manager
    }

    pub(crate) fn get_update_frame<T: 'static + Component>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        *self.last_updated_map.get(&type_id).unwrap()
    }

    pub(crate) fn get_frame(&self) -> u64 {
        self.frame
    }

    pub(crate) fn borrow_component_manager_pair_mut<
        T1: 'static + Component,
        T2: 'static + Component,
    >(
        &self,
        entity_id: usize,
    ) -> Option<(&mut T1, &mut T2)> {
        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();

        let manager1: &mut ComponentManager<T1> =
            cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2: &mut ComponentManager<T2> =
            cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());

        let t1 = manager1.borrow_components_mut(entity_id).unwrap();
        let t2 = manager2.borrow_components_mut(entity_id).unwrap();

        Some((t1, t2))
    }

    pub(crate) fn step_frame(&mut self) {
        self.frame += 1;
    }

    pub(crate) fn borrow_component<T: Component + 'static>(&self, entity_id: usize) -> Option<&T> {
        match self.has_component_manager::<T>() {
            true => self
                .borrow_component_manager::<T>()
                .borrow_component(entity_id),
            false => None,
        }
    }

    fn has_component_manager<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.manager_map.contains_key(&type_id)
    }

    pub(crate) fn borrow_components<T: Component + 'static>(&self) -> Option<&Vec<T>> {
        match self.has_component_manager::<T>() {
            true => self.borrow_component_manager::<T>().borrow_components(),
            false => None,
        }
    }

    pub(crate) fn borrow_component_mut<T: Component + 'static>(
        &mut self,
        entity_id: usize,
    ) -> Option<&mut T> {
        match self.has_component_manager::<T>() {
            true => self
                .borrow_component_manager_mut::<T>()
                .borrow_component_mut(entity_id),
            false => None,
        }
    }
}

#[allow(clippy::mut_from_ref, clippy::borrowed_box)]
fn cast_manager_mut_unsafe<T: 'static + Component>(
    boxed_component: &Box<dyn ComponentManagerTrait>,
) -> &mut ComponentManager<T> {
    // Deref box ref to box, and then deref box internal value,
    // then taking a reference to the value in the box
    let dyn_manager = &**boxed_component;

    // cast the dyn ComponentManagerTrait to an actual ComponentManager<T>
    // based on T value
    let manager: &ComponentManager<T> = cast_manager(dyn_manager);

    // Casting our ComponentManager<T> into a raw mutable pointer
    let ptr = manager as *const ComponentManager<T> as *mut ComponentManager<T>;

    // unsafely transmute our raw mutable pointer
    // to a mutable ref of a ComponentManager<T>
    let manager: &mut ComponentManager<T> = unsafe { transmute(&mut *ptr) };
    manager
}

fn cast_manager<T: 'static + Component>(
    unknown: &dyn ComponentManagerTrait,
) -> &ComponentManager<T> {
    unknown
        .as_any()
        .downcast_ref::<ComponentManager<T>>()
        .unwrap()
}

fn cast_manager_mut<T: 'static + Component>(
    unknown: &mut dyn ComponentManagerTrait,
) -> &mut ComponentManager<T> {
    unknown
        .as_any_mut()
        .downcast_mut::<ComponentManager<T>>()
        .unwrap()
}
