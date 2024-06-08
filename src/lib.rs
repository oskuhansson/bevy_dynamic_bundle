use bevy::ecs::system::{
    EntityCommands, EntityCommand
};
use bevy::prelude::*;
//use std::marker::PhantomData;

fn insert<T: Bundle>(bundle: T) -> impl DynEntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Some(mut entity) = world.get_entity_mut(entity) {
            entity.insert(bundle);
        } else {
            panic!("error[B0003]: Could not insert a bundle (of type `{}`) for entity {:?} because it doesn't exist in this World.", std::any::type_name::<T>(), entity);
        }
    }
}

/* 
pub struct DynamicWithEntity {
    bundle_fn: Box<dyn>
}
*/

pub trait DynEntityCommand<Marker = ()>: Send + 'static {
    /// Executes this command for the given [`Entity`].
    fn apply(self: Box<Self>, id: Entity, world: &mut World);
    
    /* 
    /// Returns a [`Command`] which executes this [`DynEntityCommand`] for the given [`Entity`].
    fn with_entity(self: Box<Self>, id: Entity) -> DynWithEntity<Marker> 
    where
        Self: DynEntityCommand
    {
        DynWithEntity {
            cmd: self,
            id,
            marker: PhantomData,
        }
    }
    */
}
/* 
pub struct DynWithEntity<Marker> {
    cmd: Box<dyn DynEntityCommand>,
    id: Entity,
    marker: PhantomData<fn() -> Marker>,
}

impl<M> Command for DynWithEntity<M>
where
    M: 'static,
{
    #[inline]
    fn apply(self, world: &mut World) {
        self.cmd.apply(self.id, world);
    }
}
*/

impl<F> DynEntityCommand for F
where
    F: FnOnce(Entity, &mut World) + Send + 'static,
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


pub struct DynamicBundel {
    bundle_fn: Box<dyn DynEntityCommand>
}

impl DynamicBundel {
    fn new<T: Bundle>(bundle: T) -> DynamicBundel {
        DynamicBundel {
            bundle_fn: Box::new(insert(bundle))
        }
    }
}

impl<T: Bundle> From<T> for DynamicBundel {
    fn from(bundle: T) -> Self {
        DynamicBundel::new(bundle)
    }
}

trait DynamicInsert<'a> {
    fn insert_dynamic_bundle(&mut self, dyn_bundel: DynamicBundel) -> &mut EntityCommands<'a>;
}

impl<'a> DynamicInsert<'a> for EntityCommands<'a> {
    fn insert_dynamic_bundle(&mut self, dyn_bundel: DynamicBundel) -> &mut EntityCommands<'a> {
        self.add(dyn_bundel.bundle_fn);
        self
    }
}





#[cfg(test)]
mod tests {
    //use bevy::{ecs::world, transform::commands};

    use super::*;

    #[test]
    fn simple_dyn_bundle_test() {
        #[derive(Component)]
        struct ComponentA;

        App::new().add_systems(Startup, (setup, query).chain());

        fn setup(mut commands: Commands) {
            let dyn_bundle = DynamicBundel::new(ComponentA);

            commands.spawn(()).insert_dynamic_bundle(dyn_bundle);
            
        }

        fn query(components: Query<&ComponentA>) {
            components.get_single().unwrap();
        }

    }
}
