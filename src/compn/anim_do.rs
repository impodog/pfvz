use crate::prelude::*;

pub(super) struct CompnAnimDoPlugin;

impl Plugin for CompnAnimDoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_animation, work_atd));
    }
}

/// Starts an animation, after playing, then run a certain system
/// Defaults to do nothing
#[derive(Component, Debug, Clone)]
pub struct AnimationThenDo {
    pub anim: Arc<sprite::FrameArr>,
    pub work: SystemId<Entity>,
}
impl Default for AnimationThenDo {
    fn default() -> Self {
        Self {
            anim: Default::default(),
            work: compn::default::system_do_nothing.read().unwrap().unwrap(),
        }
    }
}
#[derive(Component, Debug, Clone)]
struct AtdImpl {
    prev: Arc<sprite::FrameArr>,
}

fn start_animation(
    mut commands: Commands,
    mut q_atd: Query<(Entity, &AnimationThenDo, &mut sprite::Animation), Added<AnimationThenDo>>,
) {
    q_atd.iter_mut().for_each(|(entity, atd, mut anim)| {
        commands.entity(entity).try_insert(AtdImpl {
            prev: anim.frames(),
        });
        anim.reset(atd.anim.clone());
    });
}

fn work_atd(
    mut commands: Commands,
    mut q_atd: Query<(Entity, &AnimationThenDo, &AtdImpl, &mut sprite::Animation)>,
) {
    let work = RwLock::new(Vec::new());
    q_atd
        .par_iter_mut()
        .for_each(|(entity, atd, atd_impl, mut anim)| {
            if anim.just_finished() {
                work.write().unwrap().push((atd.work, entity));
                anim.reset(atd_impl.prev.clone());
            }
        });
    let work = RwLock::into_inner(work).unwrap();
    for (work, entity) in work.into_iter() {
        commands.run_system_with_input(work, entity);
        commands.entity(entity).remove::<AnimationThenDo>();
    }
}
