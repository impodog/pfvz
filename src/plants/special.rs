use crate::prelude::*;

pub(super) struct PlantsSpecialPlugin;

impl Plugin for PlantsSpecialPlugin {
    fn build(&self, app: &mut App) {
        initialize(&grave_systems);
        initialize(&crater_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(OnEnter(info::PlayStates::Gaming), (add_grave_timer,));
        app.add_systems(
            Update,
            (auto_spawn_grave, crater_tick).run_if(when_state!(gaming)),
        );
        *grave_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_grave),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *grave_spawn_anywhere.write().unwrap() = Some(app.register_system(spawn_grave_any));
        *crater_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_crater),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *ice_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_ice),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems grave_systems);
game_conf!(systems crater_systems);
game_conf!(systems ice_systems);
game_conf!(pub system grave_spawn_anywhere, ());

fn spawn_grave(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&GRAVE).unwrap();
    let anim = match rand::thread_rng().gen_range(1..=2) {
        1 => plants.grave1.clone(),
        2 => plants.grave2.clone(),
        _ => unreachable!(),
    };
    commands.spawn((
        game::Plant,
        // Avoids zombie damage
        game::NotPlanted,
        creature.clone(),
        pos,
        sprite::Animation::new(anim),
        creature.hitbox,
        game::Health::from(factors.grave.health),
        game::LayerDisp(-0.01),
        SpriteBundle::default(),
    ));
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct GraveTimer(pub Timer);

fn add_grave_timer(mut commands: Commands, factors: Res<plants::PlantFactors>) {
    commands.insert_resource(GraveTimer(Timer::new(
        Duration::from_secs_f32(factors.grave.cooldown),
        TimerMode::Repeating,
    )));
}

fn spawn_grave_any(
    mut commands: Commands,
    level: Res<level::Level>,
    layout: Res<game::PlantLayout>,
) {
    let size = level.config.layout.size();
    // This randomly selects an unused tile to spawn
    for _ in 0..10 {
        let (x, y) = (
            rand::thread_rng().gen_range(0..size.0),
            rand::thread_rng().gen_range(0..size.1),
        );
        let pos = level.config.layout.coordinates_to_position(x, y);
        let index = level.config.layout.position_3d_to_index(&pos);
        let pos = game::LogicPosition::from_base(pos);
        if layout
            .plants
            .get(index)
            .is_some_and(|list| list.read().unwrap().is_empty())
        {
            commands.run_system_with_input(grave_systems.read().unwrap().unwrap().spawn, pos);
            break;
        }
    }
}

fn auto_spawn_grave(
    mut commands: Commands,
    level: Res<level::Level>,
    mut timer: ResMut<GraveTimer>,
    time: Res<config::FrameTime>,
) {
    if level.config.has_grave() {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.run_system(grave_spawn_anywhere.read().unwrap().unwrap());
        }
    }
}

fn spawn_crater(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&CRATER).unwrap();
    commands.spawn((
        game::Plant,
        game::NotPlanted,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.crater.clone()),
        creature.hitbox,
        CraterTimer(Timer::new(
            Duration::from_secs_f32(factors.crater.cooldown),
            TimerMode::Once,
        )),
        game::Health::from(factors.crater.health),
        SpriteBundle::default(),
    ));
}

fn spawn_ice(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&ICE).unwrap();
    commands.spawn((
        game::Plant,
        game::NotPlanted,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.ice.clone()),
        creature.hitbox,
        CraterTimer(Timer::new(
            Duration::from_secs_f32(factors.ice.cooldown),
            TimerMode::Once,
        )),
        game::Health::from(factors.ice.health),
        SpriteBundle::default(),
    ));
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct CraterTimer(pub Timer);

fn crater_tick(
    mut commands: Commands,
    mut q_crater: Query<(Entity, &game::Overlay, &mut CraterTimer)>,
) {
    q_crater
        .iter_mut()
        .for_each(|(entity, overlay, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                commands.entity(entity).despawn_recursive();
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
            systems: grave_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .grave1
                .frames
                .first()
                .expect("Empty animation grave1")
                .clone(),
            cost: factors.grave.cost,
            cooldown: factors.grave.cooldown,
            hitbox: factors.grave.self_box,
            flags: level::CreatureFlags::GRAVE,
        }));
        map.insert(GRAVE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: crater_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .crater
                .frames
                .first()
                .expect("Empty animation crater")
                .clone(),
            cost: factors.crater.cost,
            cooldown: factors.crater.cooldown,
            hitbox: factors.crater.self_box,
            flags: level::CreatureFlags::CRATER,
        }));
        map.insert(CRATER, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: ice_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .ice
                .frames
                .first()
                .expect("Empty animation ice")
                .clone(),
            cost: factors.ice.cost,
            cooldown: factors.ice.cooldown,
            hitbox: factors.ice.self_box,
            flags: level::CreatureFlags::CRATER,
        }));
        map.insert(ICE, creature);
    }
}
