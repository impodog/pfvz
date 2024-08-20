use crate::prelude::*;

pub(super) struct PlantsPeaPlugin;

impl Plugin for PlantsPeaPlugin {
    fn build(&self, app: &mut App) {
        initialize(&peashooter_systems);
        initialize(&snow_pea_systems);
        initialize(&repeater_systems);
        initialize(&snow_pea_after);
        app.add_systems(PostStartup, (init_config,));
        *peashooter_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_peashooter),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *snow_pea_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_snow_pea),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *snow_pea_after.write().unwrap() = Some(app.register_system(add_snow));
        *repeater_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_repeater),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *threepeater_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_threepeater),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(projectile ProjectilePea);
game_conf!(shooter PeashooterShooter);
game_conf!(systems peashooter_systems);
game_conf!(projectile ProjectileSnow);
game_conf!(shooter SnowPeaShooter);
game_conf!(systems snow_pea_systems);
game_conf!(system snow_pea_after, Entity);
// We'll reuse the same projectile pea
game_conf!(shooter RepeaterShooter);
game_conf!(systems repeater_systems);
game_conf!(shooter ThreepeaterShooter);
game_conf!(systems threepeater_systems);

fn spawn_peashooter(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<PeashooterShooter>,
) {
    let creature = map.get(&PEASHOOTER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.peashooter.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.peashooter.health),
        SpriteBundle::default(),
    ));
}

fn spawn_snow_pea(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<SnowPeaShooter>,
) {
    let creature = map.get(&SNOW_PEA).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.snow_pea.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.snow_pea.health),
        SpriteBundle::default(),
    ));
}

fn add_snow(In(entity): In<Entity>, mut commands: Commands, factors: Res<plants::PlantFactors>) {
    commands.entity(entity).insert(compn::SnowyProjectile {
        snow: compn::Snow::from(factors.snow_pea.snow),
    });
}

fn spawn_repeater(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<RepeaterShooter>,
) {
    let creature = map.get(&REPEATER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.repeater.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.repeater.health),
        SpriteBundle::default(),
    ));
}

fn spawn_threepeater(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<ThreepeaterShooter>,
) {
    let creature = map.get(&THREEPEATER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.threepeater.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.threepeater.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let pea = Arc::new(game::ProjectileShared {
        anim: plants.pea.clone(),
        hitbox: factors.peashooter.pea_box,
    });
    {
        commands.insert_resource(ProjectilePea(pea.clone()));
        commands.insert_resource(PeashooterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.peashooter.interval),
            velocity: factors.peashooter.velocity.into(),
            proj: game::Projectile {
                damage: factors.peashooter.damage,
                ..Default::default()
            },
            times: factors.peashooter.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: pea.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: peashooter_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants.peashooter_concept.clone(),
            cost: factors.peashooter.cost,
            cooldown: factors.peashooter.cooldown,
            hitbox: factors.peashooter.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(PEASHOOTER, creature);
    }

    let snow = Arc::new(game::ProjectileShared {
        anim: plants.snow.clone(),
        hitbox: factors.snow_pea.pea_box,
    });
    {
        commands.insert_resource(ProjectileSnow(snow.clone()));
        commands.insert_resource(SnowPeaShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.snow_pea.interval),
            velocity: factors.snow_pea.velocity.into(),
            proj: game::Projectile {
                damage: factors.snow_pea.damage,
                ..Default::default()
            },
            times: factors.snow_pea.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: snow.clone(),
            after: snow_pea_after.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: snow_pea_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .snow_pea
                .frames
                .first()
                .expect("Empty animation snow_pea")
                .clone(),
            cost: factors.snow_pea.cost,
            cooldown: factors.snow_pea.cooldown,
            hitbox: factors.snow_pea.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(SNOW_PEA, creature);
    }
    {
        commands.insert_resource(RepeaterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.repeater.interval),
            velocity: factors.repeater.velocity.into(),
            proj: game::Projectile {
                damage: factors.repeater.damage,
                ..Default::default()
            },
            times: factors.repeater.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: pea.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: repeater_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .repeater
                .frames
                .first()
                .expect("Empty animation repeater")
                .clone(),
            cost: factors.repeater.cost,
            cooldown: factors.repeater.cooldown,
            hitbox: factors.repeater.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(REPEATER, creature);
    }
    {
        commands.insert_resource(ThreepeaterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.threepeater.interval),
            velocity: factors.threepeater.velocity.into(),
            proj: game::Projectile {
                damage: factors.threepeater.damage,
                range: game::PositionRange {
                    y: -1.5..1.5,
                    ..Default::default()
                },
                ..Default::default()
            },
            start: vec![
                (game::Position::new_xy(0.0, 0.0), 0.0),
                (game::Position::new_xy(0.0, 1.0), 0.0),
                (game::Position::new_xy(0.0, -1.0), 0.0),
            ],
            times: factors.threepeater.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: pea.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: threepeater_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .threepeater
                .frames
                .first()
                .expect("Empty animation threepeater")
                .clone(),
            cost: factors.threepeater.cost,
            cooldown: factors.threepeater.cooldown,
            hitbox: factors.threepeater.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(THREEPEATER, creature);
    }
}
