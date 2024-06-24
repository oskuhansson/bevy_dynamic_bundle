# bevy_dynamic_bundle
 A simple library for providing dynamic bundles to bevy

use DynamicBundel::new() to create a dynamic bundle that can be used by dyn_spawn or dyn_insert.

Here is some example code:
```rust
use bevy_dynamic_bundle::prelude::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
struct ComponentA(i32);

fn main() {
    App::new().add_systems(Startup, (setup, query).chain()).run();
}

fn setup(mut commands: Commands) {
    let dyn_bundle = DynamicBundel::new(ComponentA(2));

    commands.dyn_spawn(dyn_bundle);

            
}

fn query(components: Query<&ComponentA>) {
    assert_eq!(2 ,components.get_single().unwrap().0);
}
```
