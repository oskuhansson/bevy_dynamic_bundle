use::bevy_dynamic_bundle::prelude::*;
use::bevy::prelude::*;

#[derive(Component, Clone)]
struct Spawner(DynamicBundel);

#[derive(Component, Clone)]
struct ComponentA(i32);

fn main() {
    App::new().add_systems(Startup, (setup, spawn, query).chain()).run();
}

fn setup(mut commands: Commands) {
    let dyn_bundle = DynamicBundel::new(ComponentA(2));

    commands.spawn(Spawner(dyn_bundle));

            
}

fn spawn(mut commands: Commands, spawner_q: Query<&Spawner>) {
    let spawner = spawner_q.get_single().unwrap();
    commands.dyn_spawn(spawner.0.clone());
}

fn query(components: Query<&ComponentA>) {
    assert_eq!(2 ,components.get_single().unwrap().0);
}
