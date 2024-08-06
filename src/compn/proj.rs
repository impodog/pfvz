use crate::prelude::*;

pub(super) struct CompnProjPlugin;

impl Plugin for CompnProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (despawn, proj_action).chain().run_if(when_state!(gaming)),
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

fn proj_action(
    mut commands: Commands,
    mut e_proj: EventReader<game::ProjectileAction>,
    q_proj: Query<&game::Projectile>,
    mut queue: ResMut<DespawnQueue>,
) {
    e_proj.read().for_each(|action| {
        let ok = match action {
            game::ProjectileAction::Damage(entity, _other) => {
                if let Ok(proj) = q_proj.get(*entity) {
                    if proj.instant {
                        queue.push(*entity);
                        if let Some(mut commands) = commands.get_entity(*entity) {
                            // Prevents re-collision
                            // This used to be a bug! So damage of plants may seem higher than they
                            // should (twice collision)
                            commands.remove::<game::HitBox>();
                        }
                    }
                    true
                } else {
                    false
                }
            }
        };
        if !ok {
            // This is very annoying when a projectile hurts multiple targets, so it's turned off
            // warn!("Unable to execute projectile action: {:?}", action);
        }
    });
}
