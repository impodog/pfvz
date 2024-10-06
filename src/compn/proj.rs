use crate::prelude::*;

pub(super) struct CompnProjPlugin;

impl Plugin for CompnProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                despawn,
                (
                    proj_action,
                    proj_timer_tick,
                    despawn_by_hit_roof,
                    projectile_consume,
                ),
            )
                .chain()
                .run_if(when_state!(gaming)),
        );
        app.init_resource::<ProjDespawnQueue>();
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct ProjDespawnQueue(BTreeSet<Entity>);

fn despawn(mut commands: Commands, mut queue: ResMut<ProjDespawnQueue>) {
    if !queue.is_empty() {
        let q = std::mem::take(&mut queue.0);
        for entity in q.into_iter() {
            if let Some(commands) = commands.get_entity(entity) {
                commands.despawn_recursive();
            }
        }
    }
}

fn despawn_by_hit_roof(
    q_proj: Query<(Entity, &game::LogicPosition, &game::Position), With<game::Projectile>>,
    mut queue: ResMut<ProjDespawnQueue>,
    level: Res<level::Level>,
) {
    q_proj.iter().for_each(|(entity, logic, pos)| {
        let (x, _y) = level
            .config
            .layout
            .position_3d_to_coordinates(logic.base_raw());
        if pos.z < level.config.layout.get_disp(x) {
            queue.insert(entity);
        }
    });
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct ProjectileTimer(Timer);

fn proj_action(
    commands: ParallelCommands,
    mut e_proj: EventReader<game::ProjectileAction>,
    q_proj: Query<&game::Projectile>,
    queue: ResMut<ProjDespawnQueue>,
) {
    let queue = RwLock::new(queue);
    e_proj.par_read().for_each(|action| {
        let ok = match action {
            game::ProjectileAction::Damage(_entity, _other) => true,
            game::ProjectileAction::Consumed(entity) => {
                let ok = if let Ok(proj) = q_proj.get(*entity) {
                    if proj.time.as_millis() == 0 {
                        if commands
                            .command_scope(|mut commands| commands.get_entity(*entity).is_some())
                        {
                            queue.write().unwrap().insert(*entity);
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
                        commands.insert(game::DeadProjectile);
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

/// Given the projectile on consumption, do some work
#[derive(Component, Deref, DerefMut)]
pub struct ProjectileConsume(pub SystemId<Entity>);

fn projectile_consume(
    commands: ParallelCommands,
    mut e_action: EventReader<game::ProjectileAction>,
    q_proj: Query<&ProjectileConsume>,
) {
    e_action.par_read().for_each(|action| {
        if let game::ProjectileAction::Consumed(entity) = action {
            if let Ok(consume) = q_proj.get(*entity) {
                commands.command_scope(|mut commands| {
                    commands.run_system_with_input(**consume, *entity);
                });
            }
        }
    });
}
