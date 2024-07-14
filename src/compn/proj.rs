use crate::prelude::*;

pub(super) struct CompnProjPlugin;

impl Plugin for CompnProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (proj_action,).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

fn proj_action(
    mut commands: Commands,
    mut e_proj: EventReader<game::ProjectileAction>,
    q_proj: Query<&game::Projectile>,
) {
    e_proj.read().for_each(|action| {
        let ok = match action {
            game::ProjectileAction::Damage(entity) => {
                if let Ok(proj) = q_proj.get(*entity) {
                    if proj.instant {
                        commands.entity(*entity).despawn_recursive();
                    }
                    true
                } else {
                    false
                }
            }
        };
        if !ok {
            error!("Unable to execute projectile action: {:?}", action);
        }
    });
}
