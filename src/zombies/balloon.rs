use crate::prelude::*;

pub(super) struct ZombiesBalloonPlugin;

impl Plugin for ZombiesBalloonPlugin {
    fn build(&self, app: &mut App) {
        initialize(&balloon_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        *balloon_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_balloon_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems balloon_zombie_systems);
game_conf!(walker BalloonZombieWalker);

fn spawn_balloon_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BalloonZombieWalker>,
) {
    let creature = map.get(&BALLOON_ZOMBIE).unwrap();
    let pos = pos.with_disp(pos.disp.move_z(0.8));
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::from(factors.balloon.velocity),
        sprite::Animation::new(zombies.balloon_zombie.clone()),
        compn::Dying::new(zombies.balloon_zombie_dying.clone()),
        creature.hitbox,
        compn::Walker(walker.0.clone()),
        game::Health::from(factors.balloon.self_health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(BalloonZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.balloon.interval),
        damage: factors.balloon.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: balloon_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .balloon_zombie
                .frames
                .first()
                .expect("empty animation balloon_zombie")
                .clone(),
            cost: factors.balloon.cost,
            cooldown: factors.balloon.cooldown,
            hitbox: factors.balloon.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(BALLOON_ZOMBIE, creature);
    }
}
