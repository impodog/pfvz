use crate::prelude::*;

pub(super) struct GameProjPlugin;

impl Plugin for GameProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileAction>();
        app.add_systems(
            Update,
            (test_plant_proj_zombie, test_zombie_proj_plant).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectileShared {
    pub anim: Arc<sprite::FrameArr>,
    pub hitbox: game::HitBox,
}

#[derive(Event, Debug, Clone)]
pub enum ProjectileAction {
    Damage(Entity, Entity),
    Consumed(Entity),
}

#[derive(Component, Default, Debug, Clone)]
pub struct Projectile {
    pub damage: u32,
    pub area: bool,
    pub time: Duration,
    // Range is placed in `Projectile` instead of `ShooterShared`
    // This may be used for short-ranged projectiles to disappear
    pub range: game::PositionRange,
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
        let mut consumed = false;
        if let Some(set) = collision.get(&entity) {
            for zombie in set.iter() {
                if let Ok(zombie_entity) = q_zombie.get(*zombie) {
                    e_proj.send(ProjectileAction::Damage(entity, zombie_entity));
                    e_creature.send(game::CreatureAction::Damage(
                        zombie_entity,
                        multiply_uf!(proj.damage, config.gamerule.damage.0),
                    ));
                    consumed = true;
                    // This prevents multiple damages
                    if !proj.area {
                        break;
                    }
                }
            }
        }
        if consumed {
            e_proj.send(ProjectileAction::Consumed(entity));
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
        let mut consumed = false;
        if let Some(set) = collision.get(&entity) {
            for plant in set.iter() {
                if let Ok(plant_entity) = q_plant.get(*plant) {
                    e_proj.send(ProjectileAction::Damage(entity, plant_entity));
                    e_creature.send(game::CreatureAction::Damage(plant_entity, proj.damage));
                    consumed = true;
                    if !proj.area {
                        break;
                    }
                }
            }
        }
        if consumed {
            e_proj.send(ProjectileAction::Consumed(entity));
        }
    });
}
