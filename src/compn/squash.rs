use crate::prelude::*;

pub(super) struct CompnSquashPlugin;

impl Plugin for CompnSquashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (squash_test, goto_peak));
    }
}

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct SquashStatus {
    #[deref]
    pub target: Option<game::Position>,
    pub timer: Timer,
    pub peak: bool,
}

fn squash_test(
    mut q_squash: Query<(&mut SquashStatus, &mut game::Velocity, &game::Position)>,
    q_zombie: Query<&game::Position, With<game::Zombie>>,
    factors: Res<plants::PlantFactors>,
) {
    q_squash
        .par_iter_mut()
        .for_each(|(mut status, mut velocity, pos)| {
            if status.is_none() {
                let range = game::PositionRange::from(factors.squash.range) + *pos;
                for zombie_pos in q_zombie.iter() {
                    if range.contains(zombie_pos) {
                        status.target = Some(*zombie_pos);
                        velocity.x = (zombie_pos.x - pos.x) / factors.squash.time;
                        velocity.z = factors.squash.jump_height / factors.squash.time * 2.0;
                        status.timer = Timer::new(
                            Duration::from_secs_f32(factors.squash.time),
                            TimerMode::Once,
                        );
                        break;
                    }
                }
            }
        });
}

fn goto_peak(
    commands: ParallelCommands,
    mut q_squash: Query<(
        Entity,
        &game::Overlay,
        &mut SquashStatus,
        &mut game::Velocity,
    )>,
    factors: Res<plants::PlantFactors>,
    collision: Res<game::Collision>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_plant: Query<(), With<game::Plant>>,
    mut action: EventWriter<game::CreatureAction>,
) {
    let events = RwLock::new(Vec::new());
    q_squash
        .par_iter_mut()
        .for_each(|(entity, overlay, mut status, mut velocity)| {
            if status.is_some() {
                status.timer.tick(overlay.delta());
                if status.timer.finished() {
                    if let Some(coll) = collision.get(&entity) {
                        if q_plant.get(entity).is_ok() {
                            coll.iter().for_each(|zombie| {
                                if q_zombie.get(*zombie).is_ok() {
                                    events.write().unwrap().push(game::CreatureAction::Damage(
                                        *zombie,
                                        factors.squash.damage,
                                    ));
                                }
                            });
                        } else {
                            coll.iter().for_each(|plant| {
                                if q_plant.get(*plant).is_ok() {
                                    events.write().unwrap().push(game::CreatureAction::Damage(
                                        *plant,
                                        factors.squash.damage,
                                    ));
                                }
                            });
                        }
                        commands.command_scope(|mut commands| {
                            commands.entity(entity).despawn_recursive();
                        });
                    }
                } else if !status.peak && status.timer.remaining_secs() <= factors.squash.time / 2.0
                {
                    velocity.z = -velocity.z;
                    status.peak = true;
                }
            }
        });
    let events = RwLock::into_inner(events).unwrap();
    for event in events.into_iter() {
        action.send(event);
    }
}
