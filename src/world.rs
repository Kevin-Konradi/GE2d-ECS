use crate::ComponentVec;

pub(crate) struct World {
    pub(crate) entities_count: usize,
    pub(crate) component_vecs: Vec<Box<dyn ComponentVec>>
}
impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    /// create a new Entity and return the ID which is a `usize`
    pub fn create_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static> (
        &mut self,
        entity: usize,
        component: ComponentType
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<Vec<Option<ComponentType>>>() {
                component_vec[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
            self.component_vecs.push(Box::new(new_component_vec));
    }

    fn borrow_component_vec<ComponentType: 'static> (&self) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<Vec<Option<ComponentType>>>() {
                return Some(component_vec);
            }
        }

        None
    }
}

/*pub struct EntityBuilder<'a> {
    world: &'a World,
}

impl<'a> EntityBuilder<'a> {
    pub(crate) fn new(world: &mut World) -> Self {
        EntityBuilder { 
            world: &world,
        }
    }

    pub fn add_component<ComponentType: 'static>(
        &'a mut self,
        entity: usize,
        component: ComponentType
    ) -> &'a mut EntityBuilder {

        for component_vec in self.world.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<Vec<Option<ComponentType>>>() {
                component_vec[entity] = Some(component);
                break;
            }
        }

        self
    }

    pub(crate) fn build(&mut self) -> usize {
        let entity_id = self.world.entities_count;
        for component_vec in self.world.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.world.entities_count += 1;
        entity_id
    }
}*/