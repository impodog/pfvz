use crate::prelude::*;

pub(super) struct CompnInstantPlugin;

impl Plugin for CompnInstantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (instant_work,));
    }
}

/// Allows doing a work after a certain time
/// This does not despawn any entities or remove the component
#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Instant {
    #[deref]
    pub timer: Timer,
    pub work: SystemId<Entity>,
}
impl Instant {
    pub fn new(duration: Duration, work: SystemId<Entity>) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Once),
            work,
        }
    }
}

fn instant_work(
    mut commands: Commands,
    mut q_instant: Query<(Entity, &game::Overlay, &mut Instant)>,
) {
    let work = RwLock::new(Vec::new());
    q_instant
        .par_iter_mut()
        .for_each(|(entity, overlay, mut instant)| {
            instant.tick(overlay.delta());
            if instant.just_finished() {
                work.write().unwrap().push((instant.work, entity));
            }
        });
    let work = RwLock::into_inner(work).unwrap();
    for (work, entity) in work.into_iter() {
        commands.run_system_with_input(work, entity);
    }
}
