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
pub struct ProjectileInfo {
    pub instant: bool,
    pub damage: u32,
}

#[derive(Event, Debug, Clone)]
pub enum ProjectileAction {
    Damage(Entity),
}

#[derive(Component, Debug, Clone)]
pub struct Projectile {
    pub info: Arc<ProjectileInfo>,
}

fn test_plant_proj_zombie(
    config: Res<config::Config>,
    collision: Res<game::Collision>,
    mut e_proj: EventWriter<ProjectileAction>,
    mut e_creature: EventWriter<game::CreatureAction>,
    q_proj: Query<(Entity, &Projectile), With<game::PlantRelevant>>,
    q_zombie: Query<(), With<game::Zombie>>,
) {
    q_proj.iter().for_each(|(entity, proj)| {
        if let Some(set) = collision.get(&entity) {
            set.iter().for_each(|zombie| {
                if let Ok(()) = q_zombie.get(*zombie) {
                    e_proj.send(ProjectileAction::Damage(entity));
                    e_creature.send(game::CreatureAction::Damage(
                        entity,
                        multiply!(proj.info.damage, config.gamerule.damage.0),
                    ));
                }
            });
        }
    });
}
fn test_zombie_proj_plant(
    _config: Res<config::Config>,
    collision: Res<game::Collision>,
    mut e_proj: EventWriter<ProjectileAction>,
    mut e_creature: EventWriter<game::CreatureAction>,
    q_proj: Query<(Entity, &Projectile), With<game::ZombieRelevant>>,
    q_plant: Query<(), With<game::Creature>>,
) {
    q_proj.iter().for_each(|(entity, proj)| {
        if let Some(set) = collision.get(&entity) {
            set.iter().for_each(|plant| {
                if let Ok(()) = q_plant.get(*plant) {
                    e_proj.send(ProjectileAction::Damage(entity));
                    e_creature.send(game::CreatureAction::Damage(entity, proj.info.damage));
                }
            });
        }
    });
}
