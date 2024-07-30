use crate::prelude::*;

pub(super) struct PlantsContactPlugin;

impl Plugin for PlantsContactPlugin {
    fn build(&self, app: &mut App) {
        initialize(&iceberg_lettuce_systems);
        initialize(&iceberg_lettuce_contact);
        app.add_systems(PostStartup, (init_config,));
        *iceberg_lettuce_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_iceberg_lettuce),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *iceberg_lettuce_contact.write().unwrap() =
            Some(app.register_system(iceberg_lettuce_freeze));
    }
}

game_conf!(systems iceberg_lettuce_systems);
game_conf!(system iceberg_lettuce_contact, (Entity, Entity));

fn spawn_iceberg_lettuce(
    In(pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&ICEBERG_LETTUCE).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.iceberg_lettuce.clone()),
        creature.hitbox,
        compn::Contact {
            system: iceberg_lettuce_contact.read().unwrap().unwrap(),
        },
        game::Health::from(factors.iceberg_lettuce.health),
        SpriteBundle::default(),
    ));
}

fn iceberg_lettuce_freeze(
    In((entity, enemy)): In<(Entity, Entity)>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
) {
    if let Some(mut commands) = commands.get_entity(enemy) {
        commands.insert(compn::Snow::from(factors.iceberg_lettuce.snow));
    }
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: iceberg_lettuce_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .iceberg_lettuce
                .frames
                .first()
                .expect("Empty animation iceberg_lettuce")
                .clone(),
            cost: factors.iceberg_lettuce.cost,
            cooldown: factors.iceberg_lettuce.cooldown,
            hitbox: factors.iceberg_lettuce.self_box,
        }));
        map.insert(ICEBERG_LETTUCE, creature);
    }
}
