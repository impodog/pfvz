use crate::prelude::*;

pub(super) struct PlantsInstantPlugin;

impl Plugin for PlantsInstantPlugin {
    fn build(&self, app: &mut App) {
        initialize(&ice_shroom_systems);
        initialize(&ice_shroom_work);
        app.add_systems(PostStartup, (init_config,));
        *ice_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_ice_shroom),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *ice_shroom_work.write().unwrap() = Some(app.register_system(freeze_all));
    }
}

game_conf!(systems ice_shroom_systems);
game_conf!(system ice_shroom_work, Entity);

fn spawn_ice_shroom(
    In(pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&ICE_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.ice_shroom.clone()),
        creature.hitbox,
        compn::Instant::new(
            Duration::from_secs_f32(factors.ice_shroom.interval),
            ice_shroom_work.read().unwrap().unwrap(),
        ),
        game::Health::from(factors.ice_shroom.health),
        SpriteBundle::default(),
    ));
}

fn freeze_all(
    In(entity): In<Entity>,
    mut commands: Commands,
    q_zombie: Query<Entity, With<game::Zombie>>,
    factors: Res<plants::PlantFactors>,
    mut action: EventWriter<game::CreatureAction>,
    config: Res<config::Config>,
) {
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
    q_zombie.iter().for_each(|zombie_entity| {
        if let Some(mut commands) = commands.get_entity(zombie_entity) {
            commands.try_insert(compn::Snow::from(factors.ice_shroom.snow));
            action.send(game::CreatureAction::Damage(
                zombie_entity,
                multiply_uf!(factors.ice_shroom.damage, config.gamerule.damage.0),
            ));
        }
    });
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: ice_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .ice_shroom
                .frames
                .first()
                .expect("Empty animation ice_shroom")
                .clone(),
            cost: factors.ice_shroom.cost,
            cooldown: factors.ice_shroom.cooldown,
            hitbox: factors.ice_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_CREATURE,
        }));
        map.insert(ICE_SHROOM, creature);
    }
}
