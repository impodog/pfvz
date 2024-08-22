use crate::prelude::*;

pub(super) struct GameProjPlugin;

impl Plugin for GameProjPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProjectileAction>();
        app.add_systems(
            Update,
            (
                add_projectile_impl,
                test_plant_proj_zombie,
                test_zombie_proj_plant,
            )
                .run_if(when_state!(gaming)),
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
    pub manual_consume: bool,
    pub time: Duration,
    // Range is placed in `Projectile` instead of `ShooterShared`
    // This may be used for short-ranged projectiles to disappear
    pub range: game::PositionRange,
}
#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct ProjectileImpl(pub BTreeSet<Entity>);

fn add_projectile_impl(commands: ParallelCommands, q_proj: Query<Entity, Added<Projectile>>) {
    q_proj.iter().for_each(|entity| {
        commands.command_scope(|mut commands| {
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.try_insert(ProjectileImpl::default());
            }
        })
    });
}

fn test_plant_proj_zombie(
    config: Res<config::Config>,
    collision: Res<game::Collision>,
    e_proj: EventWriter<ProjectileAction>,
    e_creature: EventWriter<game::CreatureAction>,
    mut q_proj: Query<(Entity, &Projectile, &mut ProjectileImpl), With<game::PlantRelevant>>,
    q_zombie: Query<Entity, With<game::Zombie>>,
) {
    let e_proj = Mutex::new(e_proj);
    let e_creature = Mutex::new(e_creature);
    q_proj
        .par_iter_mut()
        .for_each(|(entity, proj, mut proj_impl)| {
            let mut consumed = false;
            if let Some(set) = collision.get(&entity) {
                for zombie in set.iter() {
                    if let Ok(zombie_entity) = q_zombie.get(*zombie) {
                        let ok = if proj_impl.contains(zombie) {
                            false
                        } else {
                            proj_impl.insert(*zombie);
                            true
                        };
                        if ok {
                            e_proj
                                .lock()
                                .unwrap()
                                .send(ProjectileAction::Damage(entity, zombie_entity));
                            e_creature
                                .lock()
                                .unwrap()
                                .send(game::CreatureAction::Damage(
                                    zombie_entity,
                                    multiply_uf!(proj.damage, config.gamerule.damage.0),
                                ));
                            consumed = !proj.manual_consume;
                            // This prevents multiple damages
                            if !proj.area {
                                break;
                            }
                        }
                    }
                }
            }
            if consumed {
                e_proj
                    .lock()
                    .unwrap()
                    .send(ProjectileAction::Consumed(entity));
            }
        });
}

fn test_zombie_proj_plant(
    _config: Res<config::Config>,
    collision: Res<game::Collision>,
    e_proj: EventWriter<ProjectileAction>,
    e_creature: EventWriter<game::CreatureAction>,
    mut q_proj: Query<(Entity, &Projectile, &mut ProjectileImpl), With<game::ZombieRelevant>>,
    q_plant: Query<Entity, With<game::Plant>>,
) {
    let e_proj = Mutex::new(e_proj);
    let e_creature = Mutex::new(e_creature);
    q_proj
        .par_iter_mut()
        .for_each(|(entity, proj, mut proj_impl)| {
            let mut consumed = false;
            if let Some(set) = collision.get(&entity) {
                for plant in set.iter() {
                    if let Ok(plant_entity) = q_plant.get(*plant) {
                        let ok = if proj_impl.contains(plant) {
                            false
                        } else {
                            proj_impl.insert(*plant);
                            true
                        };
                        if ok {
                            e_proj
                                .lock()
                                .unwrap()
                                .send(ProjectileAction::Damage(entity, plant_entity));
                            e_creature
                                .lock()
                                .unwrap()
                                .send(game::CreatureAction::Damage(plant_entity, proj.damage));
                            consumed = !proj.manual_consume;
                            // This prevents multiple damages
                            if !proj.area {
                                break;
                            }
                        }
                    }
                }
            }
            if consumed {
                e_proj
                    .lock()
                    .unwrap()
                    .send(ProjectileAction::Consumed(entity));
            }
        });
}
