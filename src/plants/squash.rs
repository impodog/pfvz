use crate::prelude::*;

pub(super) struct PlantsSquashPlugin;

impl Plugin for PlantsSquashPlugin {
    fn build(&self, app: &mut App) {
        initialize(&squash_systems);
        app.add_systems(PostStartup, (init_config,));
        *squash_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_squash),
            ..Default::default()
        });
    }
}

game_conf!(systems squash_systems);

fn spawn_squash(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&SQUASH).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.squash.clone()),
        creature.hitbox,
        compn::SquashStatus::default(),
        game::Velocity::default(),
        game::Health::from(factors.squash.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: SQUASH,
            systems: squash_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .squash
                .frames
                .first()
                .expect("Empty animation squash")
                .clone(),
            cost: factors.squash.cost,
            cooldown: factors.squash.cooldown,
            hitbox: factors.squash.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
