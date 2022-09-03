use std::{path::Iter, marker::PhantomData};

trait System {
    type SystemData;

    fn run(data: Query<Self::SystemData>);
}

trait Component {}

struct Query<T> {
    items: dyn Iterator<Item = T>
}

impl<T> Iterator for Query<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}

struct World {
    systems: Vec<dyn System> 
}

struct Entity {

}