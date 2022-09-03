# GE2d-ECS

---
scribble notes

```mermaid
classDiagram
    
    class prelude {

    }

    class Dispatcher {
        <<struct>>
    }

    class World {
        <<struct>>
    }

    class Entity {
        <<struct>>
    }

    class Component {
        <<trait>>
    }
    
    class System {
        <<trait>>
        type SystemData

        fn run(data: Query<SystemData>)
    }
        
    class Query {
        <<struct>>
    }

```

```rust
impl System for PosPlayerSys {
    type SystemData = (Position, mut Player)

    fn run(data: Query<SystemData>) {
        data.iter_mut(|(...)| )
    }
}

impl Dispatcher {
    run_system(system: System) {
        let systemData = data; // SystemData-Tuple
        let query_data = components.filter(type in systemData)
        system.run(query_data)
    }
}
```
