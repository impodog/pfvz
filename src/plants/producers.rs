use crate::prelude::*;

pub(super) struct PlantsProducersPlugin;

impl Plugin for PlantsProducersPlugin {
    fn build(&self, app: &mut App) {
        initialize(&sunflower_systems);
        app.add_systems(PostStartup, (init_config,));
        *sunflower_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sunflower),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(producer SunflowerProducer);
game_conf!(systems sunflower_systems);

fn spawn_sunflower(
    In(pos): In<game::Position>,
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
            flags: level::CreatureFlags::TERRESTRIAL_CREATURE,
        }));
        map.insert(SUNFLOWER, creature);
    }
}
