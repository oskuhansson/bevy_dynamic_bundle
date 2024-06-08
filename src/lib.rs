use bevy::ecs::system::{
    EntityCommands, EntityCommand
};
use bevy::prelude::*;

use dyn_clone::DynClone;

fn insert<T: Bundle + Clone>(bundle: T) -> impl DynEntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Some(mut entity) = world.get_entity_mut(entity) {
            entity.insert(bundle);
        } else {
            panic!("error[B0003]: Could not insert a bundle (of type `{}`) for entity {:?} because it doesn't exist in this World.", std::any::type_name::<T>(), entity);
        }
    }
}

trait DynEntityCommand<Marker = ()>: DynClone + Send + 'static {
    fn apply(self: Box<Self>, id: Entity, world: &mut World);
}

impl<F> DynEntityCommand for F
where
    F: FnOnce(Entity, &mut World) + DynClone + Send + 'static,
{
    fn apply(self: Box<Self>, id: Entity, world: &mut World) {
        self(id, world);
    }
}

impl EntityCommand for Box<dyn DynEntityCommand> {
    fn apply(self: Self, id: Entity, world: &mut World) {
        self.apply(id, world);
    }
}

dyn_clone::clone_trait_object!(DynEntityCommand);
#[derive(Clone)]
pub struct DynamicBundel {
    #[allow(dead_code)]
    bundle_fn: Box<dyn DynEntityCommand>
}

impl DynamicBundel {
    pub fn new<T: Bundle + Clone>(bundle: T) -> DynamicBundel {
        DynamicBundel {
            bundle_fn: Box::new(insert(bundle))
        }
    }
}

impl<T: Bundle + Clone> From<T> for DynamicBundel {
    fn from(bundle: T) -> Self {
        DynamicBundel::new(bundle)
    }
}

#[allow(dead_code)]
pub trait DynamicInsert<'a> {
    fn dyn_insert(&mut self, dyn_bundel: DynamicBundel) -> &mut EntityCommands<'a>;
}

impl<'a> DynamicInsert<'a> for EntityCommands<'a> {
    fn dyn_insert(&mut self, dyn_bundel: DynamicBundel) -> &mut EntityCommands<'a> {
        self.add(dyn_bundel.bundle_fn);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_dyn_bundle_test() {
        #[derive(Component, Clone)]
        struct ComponentA;

        App::new().add_systems(Startup, (setup, query).chain());

        fn setup(mut commands: Commands) {
            let dyn_bundle = DynamicBundel::new(ComponentA);

            commands.spawn(()).dyn_insert(dyn_bundle.clone());
            commands.spawn(()).dyn_insert(dyn_bundle.clone());
            
        }

        fn query(components: Query<&ComponentA>) {
            components.get_single().unwrap();
        }

    }
}
