use crate::prelude::*;

pub struct CompnWalkerPlugin;

impl Plugin for CompnWalkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (walker_lock_target, walker_add_impl, walker_deal_damage),
        );
    }
}

#[derive(Debug, Clone)]
pub struct WalkerShared {
    pub interval: Duration,
    pub damage: u32,
}
#[derive(Component, Debug, Clone, Deref)]
pub struct Walker(pub Arc<WalkerShared>);

#[derive(Component, Default, Debug, Clone)]
pub struct WalkerImpl {
    timer: Timer,
    velocity: game::Velocity,
    pub target: Option<Entity>,
}
impl From<&Walker> for WalkerImpl {
    fn from(value: &Walker) -> WalkerImpl {
        Self {
            timer: Timer::new(value.interval, TimerMode::Repeating),
            ..Default::default()
        }
    }
}

#[derive(Component)]
pub struct WalkerImmunity;

fn walker_add_impl(mut commands: Commands, q_walker: Query<(Entity, &Walker), Changed<Walker>>) {
    q_walker.iter().for_each(|(entity, walker)| {
        commands.entity(entity).try_insert(WalkerImpl::from(walker));
    })
}

fn walker_lock_target(
    collision: Res<game::Collision>,
    mut q_walker: Query<(&mut WalkerImpl, &mut game::Velocity, Entity)>,
    q_plant: Query<
        &game::Position,
        (
            With<game::Plant>,
            Without<game::NotPlanted>,
            Without<WalkerImmunity>,
        ),
    >,
) {
    q_walker
        .par_iter_mut()
        .for_each(|(mut walker, mut velocity, entity)| {
            if walker.target.is_none() || walker.timer.just_finished() {
                let mut target: Option<(Entity, game::Position)> = None;
                if let Some(set) = collision.get(&entity) {
                    for entity in set {
                        if let Ok(pos) = q_plant.get(*entity) {
                            let replace = if let Some((_, prev)) = &target {
                                pos.x > prev.x || (pos.x == prev.x && pos.z > prev.z)
                            } else {
                                true
                            };
                            if replace {
                                target = Some((*entity, *pos));
                            }
                        }
                    }
                }
                let target = target.map(|(entity, _)| entity);
                if target != walker.target {
                    if walker.target.is_none() {
                        walker.velocity = std::mem::take(velocity.as_mut());
                    }
                    if target.is_none() {
                        *velocity = walker.velocity;
                    }
                    walker.target = target;
                }
            }
        });
}

fn walker_deal_damage(
    mut q_walker: Query<(
        &game::Overlay,
        &mut WalkerImpl,
        &mut game::Velocity,
        &Walker,
    )>,
    q_plant: Query<(), With<game::Plant>>,
    mut action: EventWriter<game::CreatureAction>,
    audio: Res<Audio>,
    audio_zombies: Res<assets::AudioZombies>,
) {
    // NOTE: This is not parallel! Fix?
    q_walker
        .iter_mut()
        .for_each(|(overlay, mut walker_impl, mut velocity, walker)| {
            if let Some(entity) = walker_impl.target {
                walker_impl.timer.tick(overlay.delta());
                if walker_impl.timer.just_finished() {
                    if q_plant.get(entity).is_ok() {
                        action.send(game::CreatureAction::Damage(entity, walker.damage));
                        audio.play(audio_zombies.bite.random());
                    } else {
                        *velocity = std::mem::take(&mut walker_impl.velocity);
                        walker_impl.target = None;
                    }
                }
            }
        });
}
