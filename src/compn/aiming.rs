use crate::prelude::*;

pub(super) struct CompnAimingPlugin;

impl Plugin for CompnAimingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (aiming_work, aiming_redirect, re_aim_on_change)
                .chain()
                .run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Default)]
pub struct Aiming {
    pub range: game::PositionRange,
    pub target: Option<Entity>,
    pub is_plant: bool,
}

fn aiming_work(
    mut q_aiming: Query<(Entity, &game::LogicPosition, &game::HitBox, &mut Aiming), Added<Aiming>>,
    q_plant_rel: Query<(), With<game::PlantRelevant>>,
    q_zombie: Query<Entity, With<game::Zombie>>,
    q_plant: Query<Entity, With<game::Plant>>,
    q_pos: Query<(&game::Position, &game::HitBox)>,
) {
    fn square(value: f32) -> f32 {
        value * value
    }

    q_aiming
        .par_iter_mut()
        .for_each(|(proj, logic, hitbox, mut aiming)| {
            let pos = logic.center_of(hitbox);
            let eval = |entity: &Entity| {
                q_pos
                    .get(*entity)
                    .map(|(lhs_pos, _)| {
                        square(logic.base_raw().x - lhs_pos.x) + square(pos.y - lhs_pos.y)
                    })
                    .unwrap_or(f32::INFINITY)
            };

            let is_plant = q_plant_rel.get(proj).is_ok();
            let range = aiming.range + pos;
            let filter = |zombie: &Entity| {
                q_pos
                    .get(*zombie)
                    .is_ok_and(|(pos, hitbox)| range.contains(pos, hitbox))
            };
            let compare = |lhs: &Entity, rhs: &Entity| {
                eval(lhs)
                    .partial_cmp(&eval(rhs))
                    .unwrap_or(std::cmp::Ordering::Less)
            };
            let target = if is_plant {
                q_zombie.iter().filter(filter).min_by(compare)
            } else {
                q_plant.iter().filter(filter).min_by(compare)
            };
            aiming.target = target;
            aiming.is_plant = is_plant;
        });
}

fn aiming_redirect(
    commands: ParallelCommands,
    mut q_aiming: Query<(
        Entity,
        &Aiming,
        &game::Position,
        &mut game::LogicPosition,
        &mut game::Velocity,
    )>,
    q_pos: Query<&game::Position>,
) {
    q_aiming
        .par_iter_mut()
        .for_each(|(entity, aiming, pos, mut logic, mut velocity)| {
            if let Some(target_pos) = aiming.target.and_then(|target| q_pos.get(target).ok()) {
                let diff = Vec2::new(target_pos.x - pos.x, target_pos.y - pos.y);
                let angle = diff.y.atan2(diff.x);
                let len = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();

                velocity.x = len * angle.cos();
                velocity.y = len * angle.sin();
                logic.base_raw_mut().r = angle;
            } else {
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(entity) {
                        commands.remove::<Aiming>();
                    }
                });
            }
        });
}

fn re_aim_on_change(
    commands: ParallelCommands,
    q_aiming: Query<(Entity, &Aiming)>,
    q_plant: Query<(), With<game::PlantRelevant>>,
) {
    q_aiming.iter().for_each(|(entity, aiming)| {
        let is_plant = q_plant.get(entity).is_ok();
        if aiming.is_plant ^ is_plant {
            commands.command_scope(|mut commands| {
                if let Some(mut commands) = commands.get_entity(entity) {
                    commands.try_insert(Aiming {
                        range: aiming.range,
                        is_plant,
                        ..Default::default()
                    });
                }
            });
        }
    });
}
