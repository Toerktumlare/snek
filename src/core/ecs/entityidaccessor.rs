use std::{
    any::TypeId,
    collections::{hash_map::Entry, HashMap},
};

use super::{entitymanager::EntityManager, Component};

#[derive(Default)]
pub struct EntityIdAccessor {
    cache_map: HashMap<TypeId, Vec<usize>>,
    updated_frame_map: HashMap<TypeId, u64>,
}

impl EntityIdAccessor {
    pub fn new() -> Self {
        EntityIdAccessor {
            cache_map: HashMap::new(),
            updated_frame_map: HashMap::new(),
        }
    }

    pub fn borrow_ids_for_pair<T1: 'static + Component, T2: 'static + Component>(
        &mut self,
        manager: &EntityManager,
    ) -> Option<&Vec<usize>> {
        let type_id = TypeId::of::<(T1, T2)>();
        let needs_updating = if let Entry::Vacant(e) = self.cache_map.entry(type_id) {
            e.insert(Vec::new());
            true
        } else {
            let update_frame = *self.updated_frame_map.get(&type_id).unwrap();
            manager.get_update_frame::<T1>() != update_frame
                || manager.get_update_frame::<T2>() != update_frame
        };

        if needs_updating {
            let src = &manager.borrow_entity_ids::<T1>().unwrap();
            let manager_t2 = manager.borrow_component_manager::<T2>();
            let dst = self.cache_map.get_mut(&type_id).unwrap();
            dst.clear();
            for id in src.iter() {
                if manager_t2.has(*id) {
                    dst.push(*id);
                }
            }
            self.updated_frame_map.insert(type_id, manager.get_frame());
        }
        self.cache_map.get(&type_id)
    }
}
