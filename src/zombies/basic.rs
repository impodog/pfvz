use crate::prelude::*;

pub(super) struct ZombiesBasicPlugin;

impl Plugin for ZombiesBasicPlugin {
    fn build(&self, app: &mut App) {
        initialize(&basic_zombie_systems);
        initialize(&roadcone_zombie_systems);
        initialize(&bucket_zombie_systems);
        initialize(&flag_zombie_systems);
        initialize(&screen_door_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(
            Update,
            (add_basic_zombie_arm, add_aquatic_zombie_tube).run_if(when_state!(gaming)),
        );
        *basic_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_basic_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *roadcone_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_roadcone_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *bucket_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_bucket_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *flag_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_flag_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *screen_door_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_screen_door_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(walker BasicZombieWalker);
game_conf!(systems basic_zombie_systems);
game_conf!(systems roadcone_zombie_systems);
game_conf!(breaks RoadconeBreaks);
game_conf!(systems bucket_zombie_systems);
game_conf!(breaks BucketBreaks);
game_conf!(systems flag_zombie_systems);
game_conf!(systems screen_door_zombie_systems);
game_conf!(breaks ScreenDoorBreaks);

#[derive(Component, Debug, Clone)]
pub struct BasicZombieMarker;

fn add_basic_zombie_arm(
    mut commands: Commands,
    q_basic: Query<Entity, Added<BasicZombieMarker>>,
    factors: Res<zombies::ZombieFactors>,
    zombies: Res<assets::SpriteZombies>,
) {
    q_basic.iter().for_each(|entity| {
        commands
            .spawn((
                game::Position::default(),
                game::RelativePosition::new(0.1, 0.0, 0.0, 0.0),
                factors.basic.arm_box,
                sprite::Animation::new(zombies.arm.clone()),
                game::Armor::new(factors.basic.arm_health),
                game::LayerDisp(0.1),
                SpriteBundle::default(),
            ))
            .set_parent(entity);
    });
}

fn add_aquatic_zombie_tube(
    mut commands: Commands,
    q_basic: Query<(Entity, &game::LogicPosition), Added<BasicZombieMarker>>,
    level: Res<level::Level>,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_basic.iter().for_each(|(entity, logic)| {
        let (_x, y) = level
            .config
            .layout
            .position_to_coordinates(logic.base_raw());
        match level.config.layout.get_lane(y) {
            level::TileFeature::Water => {
                commands
                    .spawn((
                        game::Position::new_xyz(0.0, 0.0, -0.2),
                        factors.tube.self_box,
                        sprite::Animation::new(zombies.tube.clone()),
                        game::LayerDisp(0.1),
                        SpriteBundle::default(),
                    ))
                    .set_parent(entity);
            }
            _ => {
                // do nothing
            }
        }
    });
}

fn spawn_basic_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
) {
    let creature = map.get(&BASIC_ZOMBIE).unwrap();
    commands.spawn((
        game::Zombie,
        BasicZombieMarker,
        creature.clone(),
        pos,
        game::Velocity::from(factors.basic.velocity),
        sprite::Animation::new(zombies.basic.clone()),
        compn::Dying::new(zombies.basic_dying.clone()),
        creature.hitbox,
        compn::Walker(walker.0.clone()),
        game::Health::from(factors.basic.self_health),
        SpriteBundle::default(),
    ));
}

fn spawn_roadcone_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
    breaks: Res<RoadconeBreaks>,
) {
    let creature = map.get(&ROADCONE_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            BasicZombieMarker,
            creature.clone(),
            pos,
            game::Velocity::from(factors.basic.velocity),
            sprite::Animation::new(zombies.basic.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.0, 0.0, 0.5, 0.1),
            factors.roadcone.roadcone_box,
            sprite::Animation::new(zombies.roadcone.clone()),
            game::Armor::new(factors.roadcone.roadcone_health),
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.1),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn spawn_bucket_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
    breaks: Res<BucketBreaks>,
) {
    let creature = map.get(&BUCKET_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            BasicZombieMarker,
            creature.clone(),
            pos,
            game::Velocity::from(factors.basic.velocity),
            sprite::Animation::new(zombies.basic.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.0, 0.0, 0.53, 0.1),
            factors.bucket.bucket_box,
            sprite::Animation::new(zombies.bucket.clone()),
            game::Armor::new(factors.bucket.bucket_health),
            game::Magnetic,
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.1),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn spawn_flag_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
) {
    let creature = map.get(&FLAG_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            BasicZombieMarker,
            creature.clone(),
            pos,
            game::Velocity::from(factors.flag.velocity),
            sprite::Animation::new(zombies.basic.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(-0.2, 0.0, 0.1, -0.1),
            factors.flag.flag_box,
            sprite::Animation::new(zombies.flag.clone()),
            game::Armor::new(factors.flag.flag_health),
            game::LayerDisp(0.1),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn spawn_screen_door_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<BasicZombieWalker>,
    breaks: Res<ScreenDoorBreaks>,
) {
    let creature = map.get(&SCREEN_DOOR_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            BasicZombieMarker,
            creature.clone(),
            pos,
            game::Velocity::from(factors.basic.velocity),
            sprite::Animation::new(zombies.basic.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(-0.1, 0.0, 0.1, 0.1),
            factors.screen_door.screen_door_box,
            sprite::Animation::new(zombies.screen_door.clone()),
            game::Armor::new(factors.screen_door.screen_door_health),
            game::Magnetic,
            compn::Breaks(breaks.0.clone()),
            compn::UnsnowParent { absolute: false },
            game::LayerDisp(0.1),
            SpriteBundle::default(),
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
    commands.insert_resource(RoadconeBreaks(Arc::new(compn::BreaksShared {
        v: vec![zombies.roadcone.clone(), zombies.roadcone_broken.clone()],
        init: factors.roadcone.roadcone_health,
    })));
    commands.insert_resource(BucketBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            zombies.bucket.clone(),
            zombies.bucket_broken.clone(),
            zombies.bucket_destroyed.clone(),
        ],
        init: factors.bucket.bucket_health,
    })));
    commands.insert_resource(ScreenDoorBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            zombies.screen_door.clone(),
            zombies.screen_door_broken.clone(),
            zombies.screen_door_destroyed.clone(),
        ],
        init: factors.screen_door.screen_door_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: basic_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .basic
                .frames
                .first()
                .expect("Empty animation basic_zombie")
                .clone(),
            cost: factors.basic.cost,
            cooldown: factors.basic.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(BASIC_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: roadcone_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies.roadcone_concept.clone(),
            cost: factors.roadcone.cost,
            cooldown: factors.roadcone.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(ROADCONE_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: bucket_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies.bucket_concept.clone(),
            cost: factors.bucket.cost,
            cooldown: factors.bucket.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(BUCKET_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: flag_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies.flag_concept.clone(),
            cost: factors.flag.cost,
            cooldown: factors.flag.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(FLAG_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: screen_door_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies.screen_door_concept.clone(),
            cost: factors.screen_door.cost,
            cooldown: factors.screen_door.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(SCREEN_DOOR_ZOMBIE, creature);
    }
}
