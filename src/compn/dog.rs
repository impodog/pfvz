use crate::prelude::*;

pub(super) struct CompnDogPlugin;

impl Plugin for CompnDogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_dog_impl, dog_work).run_if(when_state!(gaming)));
    }
}

#[derive(Clone, Default)]
pub struct DogShared {
    pub angry_velocity: game::VelocityFunctor,
    pub angry_walker: Option<Arc<compn::WalkerShared>>,
    pub angry_anim: Option<Arc<sprite::FrameArr>>,
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Dog {
    #[deref]
    pub shared: Arc<DogShared>,
    pub owner: Entity,
}

#[derive(Component, Default, Debug, Clone)]
pub struct DogImpl {
    pub angry: bool,
}

fn add_dog_impl(commands: ParallelCommands, q_dog: Query<Entity, Added<Dog>>) {
    q_dog.par_iter().for_each(|entity| {
        commands.command_scope(|mut commands| {
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.try_insert(DogImpl::default());
            }
        });
    });
}

fn dog_work(
    commands: ParallelCommands,
    mut q_dog: Query<(
        Entity,
        &mut sprite::Animation,
        &mut game::VelocityBase,
        &Dog,
        &mut DogImpl,
    )>,
    q_nothing: Query<()>,
) {
    q_dog
        .iter_mut()
        .for_each(|(entity, mut anim, mut velocity_base, dog, mut dog_impl)| {
            if !dog_impl.angry && q_nothing.get(dog.owner).is_err() {
                dog_impl.angry = true;
                if let Some(ref walker) = dog.angry_walker {
                    commands.command_scope(|mut commands| {
                        if let Some(mut commands) = commands.get_entity(entity) {
                            commands.try_insert(compn::Walker(walker.clone()));
                        }
                    });
                }
                velocity_base.replace((&dog.angry_velocity).into());
                if let Some(ref angry_anim) = dog.angry_anim {
                    anim.replace(angry_anim.clone());
                }
            }
        });
}
