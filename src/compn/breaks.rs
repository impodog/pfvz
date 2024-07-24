use crate::prelude::*;

pub(super) struct CompnBreaksPlugin;

impl Plugin for CompnBreaksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_breaks_impl, breaks_work).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Debug, Clone)]
pub struct BreaksShared {
    pub v: Vec<Arc<sprite::FrameArr>>,
    pub init: u32,
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Breaks(pub Arc<BreaksShared>);

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct BreaksImpl(pub usize);

fn add_breaks_impl(mut commands: Commands, q_breaks: Query<Entity, Added<Breaks>>) {
    q_breaks.iter().for_each(|entity| {
        commands.entity(entity).insert(BreaksImpl::default());
    });
}

fn breaks_work(
    mut q_breaks: Query<(Entity, &mut sprite::Animation, &Breaks, &mut BreaksImpl)>,
    q_armor: Query<&game::Armor, Changed<game::Armor>>,
    q_health: Query<&game::Health, Changed<game::Health>>,
) {
    q_breaks
        .par_iter_mut()
        .for_each(|(entity, mut animation, breaks, mut breaks_impl)| {
            let percentage = if let Ok(armor) = q_armor.get(entity) {
                (1.0 - armor.hp.max(1) as f32 / breaks.init as f32) * breaks.v.len() as f32
            } else if let Ok(health) = q_health.get(entity) {
                (1.0 - health.value().max(1) as f32 / breaks.init as f32) * breaks.v.len() as f32
            } else {
                return;
            };
            let percentage = (percentage as usize).min(breaks.v.len() - 1);
            if percentage != breaks_impl.0 {
                breaks_impl.0 = percentage;
                animation.replace(breaks.v.get(percentage).unwrap().clone());
            }
        });
}
