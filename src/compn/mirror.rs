use crate::prelude::*;

pub(super) struct CompnMirrorPlugin;

impl Plugin for CompnMirrorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mirror_work,).run_if(when_state!(gaming)));
    }
}

#[derive(Component)]
pub struct Mirror;

fn mirror_work(
    commands: ParallelCommands,
    mut e_proj: EventReader<game::ProjectileAction>,
    q_parent: Query<&Parent>,
    q_plant: Query<(), With<game::Plant>>,
    q_children: Query<&Children>,
    q_plant_rel: Query<(), With<game::PlantRelevant>>,
    q_mirror: Query<(), With<Mirror>>,
    q_proj: Query<&game::Projectile>,
    q_velocity: Query<&mut game::Velocity>,
    queue: ResMut<compn::ProjDespawnQueue>,
) {
    let q_velocity = Mutex::new(q_velocity);
    let queue = Mutex::new(queue);
    e_proj.par_read().for_each(|action| {
        if let game::ProjectileAction::Damage(entity, other) = action {
            if !q_children
                .get(*other)
                .is_ok_and(|children| children.iter().any(|child| q_mirror.get(*child).is_ok()))
                && q_mirror.get(*other).is_err()
            {
                return;
            }
            if !q_proj
                .get(*entity)
                .is_ok_and(|proj| !proj.area && !proj.manual_consume)
            {
                return;
            }
            let is_proj_plant = q_plant_rel.get(*entity).is_ok();
            let is_self_plant = if let Ok(parent) = q_parent.get(*other) {
                q_plant.get(parent.get()).is_ok()
            } else {
                q_plant.get(*other).is_ok()
            };
            if is_proj_plant ^ is_self_plant {
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(*entity) {
                        if is_proj_plant {
                            commands
                                .remove::<game::PlantRelevant>()
                                .try_insert(game::ZombieRelevant);
                        } else {
                            commands
                                .remove::<game::ZombieRelevant>()
                                .try_insert(game::PlantRelevant);
                        }
                        commands
                            .remove::<game::DeadProjectile>()
                            .remove::<compn::ProjectileTimer>();
                    }
                    if let Ok(mut velocity) = q_velocity.lock().unwrap().get_mut(*entity) {
                        velocity.x = -velocity.x;
                        velocity.z = -velocity.z;
                    }
                    queue.lock().unwrap().remove(entity);
                });
            }
        }
    });
}
