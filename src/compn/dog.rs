use crate::prelude::*;

pub(super) struct CompnDogPlugin;

impl Plugin for CompnDogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_dog_impl, dog_work).run_if(when_state!(gaming)));
    }
}

#[derive(Debug, Clone)]
pub struct DogShared {
    pub angry_speed: f32,
    pub angry_anim: Arc<sprite::FrameArr>,
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Dog {
    #[deref]
    pub shared: Arc<DogShared>,
    pub owner: Entity,
}

#[derive(Component, Default, Debug, Clone)]
pub struct DogImpl {
    pub angry: bool,
}

fn add_dog_impl(mut commands: Commands, q_dog: Query<Entity, Added<Dog>>) {
    q_dog.iter().for_each(|entity| {
        commands.entity(entity).insert(DogImpl::default());
    });
}

fn dog_work(
    mut commands: Commands,
    mut q_dog: Query<(
        &mut game::Overlay,
        &mut sprite::Animation,
        &Dog,
        &mut DogImpl,
    )>,
) {
    q_dog
        .iter_mut()
        .for_each(|(mut overlay, mut anim, dog, mut dog_impl)| {
            if !dog_impl.angry && commands.get_entity(dog.owner).is_none() {
                dog_impl.angry = true;
                overlay.multiply(dog.angry_speed);
                anim.replace(dog.angry_anim.clone());
            }
        });
}
