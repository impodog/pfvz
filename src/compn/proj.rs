use crate::prelude::*;

pub(super) struct CompnProjPlugin;

impl Plugin for CompnProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (despawn, (proj_action, proj_timer_tick))
                .chain()
                .run_if(when_state!(gaming)),
        );
        app.init_resource::<DespawnQueue>();
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
struct DespawnQueue(Vec<Entity>);

fn despawn(mut commands: Commands, mut queue: ResMut<DespawnQueue>) {
    if !queue.is_empty() {
        for entity in queue.drain(..) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct ProjectileTimer(Timer);

fn proj_action(
    commands: ParallelCommands,
    mut e_proj: EventReader<game::ProjectileAction>,
    q_proj: Query<&game::Projectile>,
    queue: ResMut<DespawnQueue>,
) {
    let queue = RwLock::new(queue);
    e_proj.par_read().for_each(|action| {
        let ok = match action {
            game::ProjectileAction::Damage(_entity, _other) => {
                // TODO: Any good way to handle this?
                true
            }
            game::ProjectileAction::Consumed(entity) => {
                let ok = if let Ok(proj) = q_proj.get(*entity) {
                    if proj.time.as_millis() == 0 {
                        if commands
                            .command_scope(|mut commands| commands.get_entity(*entity).is_some())
                        {
                            queue.write().unwrap().push(*entity);
                            true
                        } else {
                            false
                        }
                    } else {
                        commands.command_scope(|mut commands| {
                            if let Some(mut commands) = commands.get_entity(*entity) {
                                commands.try_insert(ProjectileTimer(Timer::new(
                                    proj.time,
                                    TimerMode::Once,
                                )));
                                true
                            } else {
                                false
                            }
                        })
                    }
                } else {
                    false
                };
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(*entity) {
                        // Removing these identifiers makes sure that the projectile is no
                        // longer tested or deals damage
                        commands.remove::<game::PlantRelevant>();
                        commands.remove::<game::ZombieRelevant>();
                    }
                });
                ok
            }
        };
        if !ok {
            // This is very annoying when a projectile hurts multiple targets, so it's turned off
            // warn!("Unable to execute projectile action: {:?}", action);
        }
    });
}

fn proj_timer_tick(
    mut commands: Commands,
    mut q_proj: Query<(Entity, &mut ProjectileTimer), With<game::Projectile>>,
    time: Res<config::FrameTime>,
) {
    q_proj.iter_mut().for_each(|(entity, mut timer)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    });
}
