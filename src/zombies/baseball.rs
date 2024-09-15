use crate::prelude::*;

pub(super) struct ZombiesBaseballPlugin;

impl Plugin for ZombiesBaseballPlugin {
    fn build(&self, app: &mut App) {
        initialize(&baseball_zombie_systems);
        initialize(&baseball_after);
        initialize(&baseball_zombie_callback);
        app.add_systems(PostStartup, (init_config,));
        *baseball_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_baseball_zombie),
            ..Default::default()
        });
        *baseball_after.write().unwrap() = Some(app.register_system(add_gravity));
        *baseball_zombie_callback.write().unwrap() =
            Some(app.register_system(baseball_zombie_play_animation));
    }
}

game_conf!(projectile ProjectileBaseball);
game_conf!(shooter BaseballZombieShooter);
game_conf!(systems baseball_zombie_systems);
game_conf!(pub system baseball_after, Entity);
game_conf!(system baseball_zombie_callback, Entity);
game_conf!(walker BaseballZombieWalker);

fn add_gravity(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(game::Gravity);
    }
}

fn spawn_baseball_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BaseballZombieWalker>,
    shooter: Res<BaseballZombieShooter>,
) {
    let creature = map.get(&BASEBALL_ZOMBIE).unwrap();
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::from(factors.baseball.velocity),
        sprite::Animation::new(zombies.baseball_zombie.clone()),
        compn::Dying::new(zombies.baseball_zombie_dying.clone()),
        creature.hitbox,
        compn::Walker(walker.0.clone()),
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.baseball.self_health),
        SpriteBundle::default(),
    ));
}

fn baseball_zombie_play_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: zombies.baseball_zombie_lob.clone(),
            ..Default::default()
        });
    }
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(BaseballZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.baseball.interval),
        damage: factors.baseball.damage,
    })));

    let baseball = Arc::new(game::ProjectileShared {
        anim: zombies.baseball.clone(),
        hitbox: factors.baseball.baseball_box,
    });
    commands.insert_resource(ProjectileBaseball(baseball.clone()));
    commands.insert_resource(BaseballZombieShooter(Arc::new(compn::ShooterShared {
        interval: Duration::from_secs_f32(factors.baseball.baseball_interval),
        velocity: factors.baseball.baseball_velocity.into(),
        proj: game::Projectile {
            damage: factors.baseball.baseball_damage,
            range: game::PositionRange::default()
                .with_inf_z()
                .with_inverted_x(),
            ..Default::default()
        },
        times: factors.baseball.times,
        require_zombie: compn::RequireZombie::No,
        predicate: Some(compn::ShooterPredicate(Arc::new(
            |(lhs_pos, lhs_creature), (rhs_pos, rhs_creature)| {
                let cmp = lhs_creature.cost.cmp(&rhs_creature.cost);
                if cmp.is_eq() {
                    lhs_pos.x.total_cmp(&rhs_pos.x)
                } else {
                    cmp
                }
            },
        ))),
        shared: baseball.clone(),
        after: baseball_after.read().unwrap().unwrap(),
        callback: baseball_zombie_callback.read().unwrap().unwrap(),
        ..Default::default()
    })));

    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: baseball_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .baseball_zombie
                .frames
                .first()
                .expect("empty animation baseball_zombie")
                .clone(),
            cost: factors.baseball.cost,
            cooldown: factors.baseball.cooldown,
            hitbox: factors.baseball.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(BASEBALL_ZOMBIE, creature);
    }
}
