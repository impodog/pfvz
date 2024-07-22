use crate::prelude::*;

pub(super) struct ZombiesBasicPlugin;

impl Plugin for ZombiesBasicPlugin {
    fn build(&self, app: &mut App) {
        initialize(&basic_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        *basic_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_basic_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(walker BasicZombieWalker);
game_conf!(systems basic_zombie_systems);

fn spawn_basic_zombie(
    In(pos): In<game::Position>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
) {
    let creature = map.get(&BASIC_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.basic.velocity),
            sprite::Animation::new(creature.anim.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::new_xy(0.1, 0.0),
            factors.basic.arm_box,
            sprite::Animation::new(zombies.arm.clone()),
            game::Armor::new(factors.basic.arm_health),
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            },
        ))
        .set_parent(entity);
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(BasicZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.basic.interval),
        damage: factors.basic.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: basic_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            anim: zombies.basic.clone(),
            cost: factors.basic.cost,
            cooldown: factors.basic.cooldown,
            hitbox: factors.basic.self_box,
        }));
        map.insert(BASIC_ZOMBIE, creature);
    }
}
