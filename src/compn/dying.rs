use crate::prelude::*;

pub(super) struct CompnDyingPlugin;

impl Plugin for CompnDyingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_dying_impl, dying_work));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Dying {
    anim: Arc<sprite::FrameArr>,
}
impl Dying {
    pub fn new(anim: Arc<sprite::FrameArr>) -> Self {
        Self { anim }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct DyingImpl {
    triggered: bool,
}

fn add_dying_impl(mut commands: Commands, q_dying: Query<Entity, Added<Dying>>) {
    q_dying.iter().for_each(|entity| {
        commands.entity(entity).insert((DyingImpl::default(),));
    });
}

fn dying_work(
    mut q_dying: Query<(
        &Dying,
        &mut DyingImpl,
        &mut game::Health,
        &mut sprite::Animation,
        &mut game::Velocity,
    )>,
) {
    q_dying.par_iter_mut().for_each(
        |(dying, mut dying_impl, mut health, mut anim, mut velocity)| {
            if dying_impl.triggered {
                health.true_decr(1)
            } else if health.is_dying() {
                dying_impl.triggered = true;
                velocity.x = 0.0;
                anim.replace(dying.anim.clone());
            }
        },
    );
}
