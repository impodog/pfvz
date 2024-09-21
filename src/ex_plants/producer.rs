use crate::prelude::*;

pub(super) struct ExPlantsProducerPlugin;

impl Plugin for ExPlantsProducerPlugin {
    fn build(&self, app: &mut App) {
        initialize(&twin_sunflower_systems);
        app.add_systems(PostStartup, (init_config,));
        *twin_sunflower_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_twin_sunflower),
            ..Default::default()
        });
    }
}

game_conf!(producer TwinSunflowerProducer);
game_conf!(systems twin_sunflower_systems);

fn spawn_twin_sunflower(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    ex_factors: Res<ex_plants::ExPlantFactors>,
    ex_plants: Res<assets::SpriteExPlants>,
    map: Res<game::CreatureMap>,
    producer: Res<TwinSunflowerProducer>,
) {
    let creature = map.get(&TWIN_SUNFLOWER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(ex_plants.twin_sunflower.clone()),
        creature.hitbox,
        compn::Producer(producer.0.clone()),
        game::Health::from(ex_factors.twin_sunflower.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    ex_plants: Res<assets::SpriteExPlants>,
    ex_factors: Res<ex_plants::ExPlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(TwinSunflowerProducer(Arc::new(compn::ProducerShared {
        interval: Duration::from_secs_f32(ex_factors.twin_sunflower.interval),
        velocity: ex_factors.twin_sunflower.velocity,
        collectible: collectible::Collectible::Sun(ex_factors.twin_sunflower.multiplier),
        times: ex_factors.twin_sunflower.times,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: TWIN_SUNFLOWER,
            systems: twin_sunflower_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_plants
                .twin_sunflower
                .frames
                .first()
                .expect("Empty animation twin_sunflower")
                .clone(),
            cost: ex_factors.twin_sunflower.cost,
            cooldown: ex_factors.twin_sunflower.cooldown,
            hitbox: ex_factors.twin_sunflower.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
