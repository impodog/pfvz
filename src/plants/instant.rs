use crate::prelude::*;

pub(super) struct PlantsInstantPlugin;

impl Plugin for PlantsInstantPlugin {
    fn build(&self, app: &mut App) {
        initialize(&ice_shroom_systems);
        initialize(&ice_shroom_work);
        app.add_systems(PostStartup, (init_config,));
        *ice_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_ice_shroom),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
        *ice_shroom_work.write().unwrap() = Some(app.register_system(freeze_all));
        *doom_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_doom_shroom),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
        *doom_shroom_work.write().unwrap() = Some(app.register_system(doom_all));
    }
}

game_conf!(systems ice_shroom_systems);
game_conf!(system ice_shroom_work, Entity);
game_conf!(systems doom_shroom_systems);
game_conf!(system doom_shroom_work, Entity);

fn spawn_ice_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&ICE_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
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
            commands.try_insert(compn::ModifySnow::Add(factors.ice_shroom.snow.into()));
            action.send(game::CreatureAction::Damage(
                zombie_entity,
                multiply_uf!(factors.ice_shroom.damage, config.gamerule.damage.0),
            ));
        }
    });
}

#[derive(Component)]
pub struct DoomShroomMarker;

fn spawn_doom_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&DOOM_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.doom_shroom.clone()),
        creature.hitbox,
        compn::Instant::new(
            Duration::from_secs_f32(factors.doom_shroom.interval),
            doom_shroom_work.read().unwrap().unwrap(),
        ),
        game::Health::from(factors.doom_shroom.health),
        DoomShroomMarker,
        SpriteBundle::default(),
    ));
}

fn doom_all(
    In(entity): In<Entity>,
    mut commands: Commands,
    q_zombie: Query<Entity, With<game::Zombie>>,
    q_pos: Query<&game::LogicPosition>,
    factors: Res<plants::PlantFactors>,
    mut action: EventWriter<game::CreatureAction>,
    config: Res<config::Config>,
) {
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
    if let Ok(pos) = q_pos.get(entity) {
        commands.run_system_with_input(plants::crater_systems.read().unwrap().unwrap().spawn, *pos);
    }
    q_zombie.iter().for_each(|zombie_entity| {
        action.send(game::CreatureAction::Damage(
            zombie_entity,
            multiply_uf!(factors.doom_shroom.damage, config.gamerule.damage.0),
        ));
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
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(ICE_SHROOM, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: doom_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .doom_shroom
                .frames
                .first()
                .expect("Empty animation doom_shroom")
                .clone(),
            cost: factors.doom_shroom.cost,
            cooldown: factors.doom_shroom.cooldown,
            hitbox: factors.doom_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(DOOM_SHROOM, creature);
    }
}
