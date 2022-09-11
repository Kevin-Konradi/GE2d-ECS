use std::cell::{RefCell, RefMut};

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

    /// add a Component of `ComponentType` to a entity, represented by an `usize`
    /// 
    /// ## example
    /// ```
    /// let entity1 = 1;
    /// let entity2 = 2;
    /// 
    /// #[derive(Component)]
    /// struct Health(usize);
    /// #[derive(Component)]
    /// struct Name(string);
    /// 
    /// world.add_component_to_entity(entity1, Health(100));
    /// world.add_component_to_entity(entity1, Name("Somebody"));
    /// world.add_component_to_entity(entity2, Health(20));
    /// ```
    pub fn add_component_to_entity<T: 'static> (
        &mut self,
        entity: usize,
        component: T
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<T>>>>() {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<T>> = Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
            self.component_vecs.push(Box::new(new_component_vec));
    }

    /// Borrow an immutable instance of `ComponentVec` to iterate over
    /// 
    /// ## example
    /// ```
    /// let data = world.borrow_component_vec::<Health>().unwrap();
    /// for health in data.iter().filter_map(|x| x.as_ref()) {
    ///     /* Do whatever you want here */
    /// }
    /// 
    /// let zip = world.borrow_component_vec::<Health>().unwrap().iter().zip(world.borrow_component_vec::<Name>().unwrap().iter());
    /// for (health, name) in zip.filter_map((|health, name|) {
    ///     Some((health.as_ref()?, name.as_ref()?))
    /// }) {
    ///     /* Do whatever you want here */
    /// }
    /// ```
    fn borrow_component_vec<T: 'static> (&self) -> Option<&Vec<Option<T>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<Vec<Option<T>>>() {
                return Some(component_vec);
            }
        }

        None
    }

    /// Borrow a mutable instance of `ComponentVec` to iterate over
    /// 
    /// ## example
    /// ```

    /// let zip = world.borrow_component_vec::<Health>().unwrap().iter_mut().zip(world.borrow_component_vec::<Name>().unwrap().iter_mut());
    /// for (health, name) in zip.filter_map((|health, name|) {
    ///     Some((health.as_mut()?, name.as_mut()?))
    /// }) {
    ///     /* Do whatever you want here */
    /// }
    /// ```
    fn borrow_component_vec_mut<T: 'static> (&self) -> Option<RefMut<Vec<Option<T>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<T>>>>() {
                return Some(component_vec.borrow_mut());
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