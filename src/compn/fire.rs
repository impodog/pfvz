use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct CompnFirePlugin;

impl Plugin for CompnFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ModifyFire>();
        app.add_systems(
            Update,
            (fire_event, fire_proj_work).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub enum ModifyFire {
    Add(Entity, FireProjectile),
}

#[derive(Component, Debug, Clone, Copy)]
pub struct FireProjectile {
    /// Additional damage multiplier on projectile
    pub additional: f32,
    /// Splash damage multiplier on projectile
    pub splash: f32,
    /// Effective range of splash
    pub range: game::PositionRange,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FireProjectileSerde {
    pub additional: f32,
    pub splash: f32,
    pub range: game::PositionRangeSerde,
}
impl From<FireProjectileSerde> for FireProjectile {
    fn from(values: FireProjectileSerde) -> Self {
        Self {
            additional: values.additional,
            splash: values.splash,
            range: values.range.into(),
        }
    }
}

fn fire_event(
    mut e_fire: EventReader<ModifyFire>,
    commands: ParallelCommands,
    q_fire: Query<&mut FireProjectile>,
) {
    let q_fire = Mutex::new(q_fire);
    e_fire.par_read().for_each(|event| match event {
        ModifyFire::Add(entity, fire) => {
            let mut q_fire = q_fire.lock().unwrap();
            if let Ok(mut original) = q_fire.get_mut(*entity) {
                original.additional += fire.additional;
                original.splash += fire.splash;
                original.range.merge(&fire.range);
            } else {
                std::mem::drop(q_fire);
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(*entity) {
                        commands.try_insert(*fire);
                    }
                });
            }
        }
    });
}

fn fire_proj_work(
    mut e_action: EventReader<game::ProjectileAction>,
    e_creature: EventWriter<game::CreatureAction>,
    q_fire: Query<
        (
            &game::Projectile,
            &FireProjectile,
            &game::ProjectileImpl,
            &game::Position,
        ),
        (Without<game::Plant>, Without<game::Zombie>),
    >,
    q_plant_rel: Query<(), (With<game::PlantRelevant>,)>,
    q_plant: Query<
        (Entity, &game::Position, &game::HitBox),
        (With<game::Plant>, Without<game::Zombie>),
    >,
    q_zombie: Query<
        (Entity, &game::Position, &game::HitBox),
        (With<game::Zombie>, Without<game::Plant>),
    >,
) {
    let e_creature = Mutex::new(e_creature);
    e_action.par_read().for_each(|action| match action {
        game::ProjectileAction::Damage(entity, other) => {
            if let Ok((proj, fire, _proj_impl, _pos)) = q_fire.get(*entity) {
                if fire.additional.abs() > f32::EPSILON {
                    e_creature
                        .lock()
                        .unwrap()
                        .send(game::CreatureAction::Damage(
                            *other,
                            multiply_uf!(proj.damage, fire.additional),
                        ));
                }
            }
        }
        game::ProjectileAction::Consumed(entity) => {
            if let Ok((proj, fire, proj_impl, pos)) = q_fire.get(*entity) {
                let range = fire.range + *pos;
                let damage = multiply_uf!(proj.damage, fire.splash);
                if q_plant_rel.get(*entity).is_ok() {
                    q_zombie
                        .par_iter()
                        .for_each(|(zombie, zombie_pos, hitbox)| {
                            if proj_impl.get(&zombie).is_none()
                                && range.contains(zombie_pos, hitbox)
                            {
                                e_creature
                                    .lock()
                                    .unwrap()
                                    .send(game::CreatureAction::Damage(zombie, damage));
                            }
                        });
                } else {
                    q_plant.par_iter().for_each(|(plant, plant_pos, hitbox)| {
                        if proj_impl.get(&plant).is_none() && range.contains(plant_pos, hitbox) {
                            e_creature
                                .lock()
                                .unwrap()
                                .send(game::CreatureAction::Damage(plant, damage));
                        }
                    });
                }
            }
        }
    });
}
