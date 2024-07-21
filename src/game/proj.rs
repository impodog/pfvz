use crate::prelude::*;

pub(super) struct GameProjPlugin;

impl Plugin for GameProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileAction>();
        app.add_systems(
            Update,
            (test_plant_proj_zombie, test_zombie_proj_plant)
                .run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Debug, Clone)]
pub struct ProjectileShared {
    pub anim: Arc<sprite::FrameArr>,
    pub hitbox: game::HitBox,
}

#[derive(Event, Debug, Clone)]
pub enum ProjectileAction {
    Damage(Entity),
}

#[derive(Component, Debug, Clone)]
pub struct Projectile {
    pub damage: u32,
    pub instant: bool,
}

fn test_plant_proj_zombie(
    config: Res<config::Config>,
    collision: Res<game::Collision>,
    mut e_proj: EventWriter<ProjectileAction>,
    mut e_creature: EventWriter<game::CreatureAction>,
    q_proj: Query<(Entity, &Projectile), With<game::PlantRelevant>>,
    q_zombie: Query<Entity, With<game::Zombie>>,
) {
    q_proj.iter().for_each(|(entity, proj)| {
        if let Some(set) = collision.get(&entity) {
            for zombie in set.iter() {
                if let Ok(zombie_entity) = q_zombie.get(*zombie) {
                    e_proj.send(ProjectileAction::Damage(entity));
                    e_creature.send(game::CreatureAction::Damage(
                        zombie_entity,
                        multiply_uf!(proj.damage, config.gamerule.damage.0),
                    ));
                    // This prevents multiple damages
                    break;
                }
            }
        }
    });
}
fn test_zombie_proj_plant(
    _config: Res<config::Config>,
    collision: Res<game::Collision>,
    mut e_proj: EventWriter<ProjectileAction>,
    mut e_creature: EventWriter<game::CreatureAction>,
    q_proj: Query<(Entity, &Projectile), With<game::ZombieRelevant>>,
    q_plant: Query<Entity, With<game::Plant>>,
) {
    q_proj.iter().for_each(|(entity, proj)| {
        if let Some(set) = collision.get(&entity) {
            set.iter().for_each(|plant| {
                if let Ok(plant_entity) = q_plant.get(*plant) {
                    e_proj.send(ProjectileAction::Damage(entity));
                    e_creature.send(game::CreatureAction::Damage(plant_entity, proj.damage));
                }
            });
        }
    });
}
