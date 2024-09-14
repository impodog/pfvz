use crate::prelude::*;

pub(super) struct ZombiesAllStarPlugin;

impl Plugin for ZombiesAllStarPlugin {
    fn build(&self, app: &mut App) {
        initialize(&all_star_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (all_star_tackle,).run_if(when_state!(gaming)));
        *all_star_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_all_star_zombie),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
    }
}

#[derive(Component, Debug)]
pub struct AllStarZombieRunning(pub bool);
impl Default for AllStarZombieRunning {
    fn default() -> Self {
        Self(true)
    }
}

game_conf!(walker AllStarZombieWalker);
game_conf!(breaks HelmetBreaks);
game_conf!(systems all_star_zombie_systems);

fn spawn_all_star_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    breaks: Res<HelmetBreaks>,
) {
    let creature = map.get(&ALL_STAR_ZOMBIE).unwrap();
    let velocity = game::Velocity::from(factors.all_star.velocity);
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.all_star.velocity_running),
            game::VelocityBase(velocity),
            sprite::Animation::new(zombies.all_star_running.clone()),
            compn::Dying::new(zombies.all_star_dying.clone()),
            creature.hitbox,
            AllStarZombieRunning::default(),
            game::Health::from(factors.all_star.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.03, 0.0, 0.35, -0.3),
            factors.all_star.helmet_box,
            sprite::Animation::new(zombies.helmet.clone()),
            game::Armor::new(factors.all_star.helmet_health),
            game::Magnetic,
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.01),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

#[allow(clippy::too_many_arguments)]
fn all_star_tackle(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    collision: Res<game::Collision>,
    walker: Res<AllStarZombieWalker>,
    mut action: EventWriter<game::CreatureAction>,
    mut q_all_star: Query<(
        Entity,
        &mut AllStarZombieRunning,
        &mut sprite::Animation,
        &mut game::Velocity,
    )>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_plant: Query<(), (With<game::Plant>, Without<game::NotPlanted>)>,
) {
    q_all_star
        .iter_mut()
        .for_each(|(entity, mut running, mut anim, mut velocity)| {
            if !running.0 {
                return;
            }
            if let Some(coll) = collision.get(&entity) {
                let ok = if q_zombie.get(entity).is_ok() {
                    if let Some(plant) = coll
                        .iter()
                        .find_map(|plant| q_plant.get(*plant).ok().map(|_| *plant))
                    {
                        action.send(game::CreatureAction::Damage(
                            plant,
                            factors.all_star.tackle_damage,
                        ));
                        true
                    } else {
                        false
                    }
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if let Some(zombie) = coll
                        .iter()
                        .find_map(|zombie| q_zombie.get(*zombie).ok().map(|_| *zombie))
                    {
                        action.send(game::CreatureAction::Damage(
                            zombie,
                            factors.all_star.tackle_damage,
                        ));
                        true
                    } else {
                        false
                    }
                };
                if ok {
                    running.0 = false;
                    anim.replace(zombies.all_star.clone());
                    commands
                        .entity(entity)
                        .insert(compn::Walker(walker.0.clone()));
                    *velocity = factors.all_star.velocity.into();
                }
            }
        });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(AllStarZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.all_star.interval),
        damage: factors.all_star.damage,
    })));
    commands.insert_resource(HelmetBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            zombies.helmet.clone(),
            zombies.helmet_broken.clone(),
            zombies.helmet_destroyed.clone(),
        ],
        init: factors.all_star.helmet_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: all_star_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies.all_star_concept.clone(),
            cost: factors.all_star.cost,
            cooldown: factors.all_star.cooldown,
            hitbox: factors.all_star.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(ALL_STAR_ZOMBIE, creature);
    }
}
