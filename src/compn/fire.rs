use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct CompnFirePlugin;

impl Plugin for CompnFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire_proj_work,));
    }
}

#[derive(Component, Debug, Clone)]
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

fn fire_proj_work(
    mut e_action: EventReader<game::ProjectileAction>,
    e_creature: EventWriter<game::CreatureAction>,
    q_fire: Query<
        (&game::Projectile, &FireProjectile, &game::Position),
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
            if let Ok((proj, fire, _pos)) = q_fire.get(*entity) {
                e_creature
                    .lock()
                    .unwrap()
                    .send(game::CreatureAction::Damage(
                        *other,
                        multiply_uf!(proj.damage, fire.additional),
                    ));
            }
        }
        game::ProjectileAction::Consumed(entity) => {
            if let Ok((proj, fire, pos)) = q_fire.get(*entity) {
                let range = fire.range.clone() + *pos;
                let damage = multiply_uf!(proj.damage, fire.splash);
                if q_plant_rel.get(*entity).is_ok() {
                    q_zombie
                        .par_iter()
                        .for_each(|(zombie, zombie_pos, hitbox)| {
                            if range.contains(zombie_pos, hitbox) {
                                e_creature
                                    .lock()
                                    .unwrap()
                                    .send(game::CreatureAction::Damage(zombie, damage));
                            }
                        });
                } else {
                    q_plant.par_iter().for_each(|(plant, plant_pos, hitbox)| {
                        if range.contains(plant_pos, hitbox) {
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
