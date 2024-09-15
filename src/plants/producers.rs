use crate::prelude::*;

pub(super) struct PlantsProducersPlugin;

impl Plugin for PlantsProducersPlugin {
    fn build(&self, app: &mut App) {
        initialize(&sunflower_systems);
        initialize(&sun_shroom_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (sun_shroom_grow,));
        *sunflower_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sunflower),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
        *sun_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sun_shroom),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
    }
}

game_conf!(producer SunflowerProducer);
game_conf!(systems sunflower_systems);
game_conf!(producer SunShroomSmallProducer);
game_conf!(producer SunShroomBigProducer);
game_conf!(systems sun_shroom_systems);

fn spawn_sunflower(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    producer: Res<SunflowerProducer>,
) {
    let creature = map.get(&SUNFLOWER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.sunflower.clone()),
        creature.hitbox,
        compn::Producer(producer.0.clone()),
        game::Health::from(factors.sunflower.health),
        SpriteBundle::default(),
    ));
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct SunShroomTimer(Timer);

fn spawn_sun_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    producer: Res<SunShroomSmallProducer>,
) {
    let creature = map.get(&SUN_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.sun_shroom_small.clone()),
        factors.sun_shroom.small_box,
        compn::Producer(producer.0.clone()),
        game::Health::from(factors.sun_shroom.health),
        SunShroomTimer(Timer::from_seconds(
            factors.sun_shroom.grow_interval,
            TimerMode::Once,
        )),
        SpriteBundle::default(),
    ));
}

fn sun_shroom_grow(
    mut q_sun_shroom: Query<(
        &game::Overlay,
        &mut SunShroomTimer,
        &mut sprite::Animation,
        &mut compn::Producer,
    )>,
    plants: Res<assets::SpritePlants>,
    big_producer: Res<SunShroomBigProducer>,
) {
    q_sun_shroom
        .par_iter_mut()
        .for_each(|(overlay, mut timer, mut anim, mut producer)| {
            if !timer.finished() {
                timer.tick(overlay.delta());
                if timer.just_finished() {
                    anim.replace(plants.sun_shroom_big.clone());
                    producer.0.clone_from(&big_producer.0);
                }
            }
        });
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(SunflowerProducer(Arc::new(compn::ProducerShared {
        interval: Duration::from_secs_f32(factors.sunflower.interval),
        velocity: factors.sunflower.velocity,
        collectible: collectible::Collectible::Sun(factors.sunflower.multiplier),
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: sunflower_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .sunflower
                .frames
                .first()
                .expect("Empty animation sunflower")
                .clone(),
            cost: factors.sunflower.cost,
            cooldown: factors.sunflower.cooldown,
            hitbox: factors.sunflower.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(SUNFLOWER, creature);
    }
    commands.insert_resource(SunShroomSmallProducer(Arc::new(compn::ProducerShared {
        interval: Duration::from_secs_f32(factors.sun_shroom.interval),
        velocity: factors.sun_shroom.velocity,
        collectible: collectible::Collectible::Sun(factors.sun_shroom.small_multiplier),
    })));
    commands.insert_resource(SunShroomBigProducer(Arc::new(compn::ProducerShared {
        interval: Duration::from_secs_f32(factors.sun_shroom.interval),
        velocity: factors.sun_shroom.velocity,
        collectible: collectible::Collectible::Sun(factors.sun_shroom.big_multiplier),
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: sun_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .sun_shroom_big
                .frames
                .first()
                .expect("Empty animation sun_shroom_small")
                .clone(),
            cost: factors.sun_shroom.cost,
            cooldown: factors.sun_shroom.cooldown,
            hitbox: factors.sun_shroom.big_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(SUN_SHROOM, creature);
    }
}
