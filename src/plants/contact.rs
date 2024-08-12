use crate::prelude::*;

pub(super) struct PlantsContactPlugin;

impl Plugin for PlantsContactPlugin {
    fn build(&self, app: &mut App) {
        initialize(&iceberg_lettuce_systems);
        initialize(&iceberg_lettuce_contact);
        initialize(&sun_bean_systems);
        initialize(&sun_bean_contact);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (sun_bean_tick,));
        *iceberg_lettuce_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_iceberg_lettuce),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *iceberg_lettuce_contact.write().unwrap() =
            Some(app.register_system(iceberg_lettuce_freeze));
        *sun_bean_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sun_bean),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *sun_bean_contact.write().unwrap() = Some(app.register_system(sun_bean_collect));
    }
}

game_conf!(systems iceberg_lettuce_systems);
game_conf!(system iceberg_lettuce_contact, (Entity, Entity));
game_conf!(systems sun_bean_systems);
game_conf!(system sun_bean_contact, (Entity, Entity));

fn spawn_iceberg_lettuce(
    In(pos): In<game::LogicPosition>,
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
        commands.insert(compn::ModifySnow::Add(compn::Snow::from(
            factors.iceberg_lettuce.snow,
        )));
    }
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
}

fn spawn_sun_bean(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&SUN_BEAN).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.sun_bean.clone()),
        creature.hitbox,
        compn::Contact {
            system: sun_bean_contact.read().unwrap().unwrap(),
        },
        SunBeanValue {
            timer: Timer::new(
                Duration::from_secs_f32(factors.sun_bean.interval),
                TimerMode::Repeating,
            ),
            value: 25,
        },
        game::Health::from(factors.sun_bean.health),
        SpriteBundle::default(),
    ));
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct SunBeanValue {
    #[deref]
    timer: Timer,
    value: u32,
}

fn sun_bean_collect(
    In((entity, _enemy)): In<(Entity, Entity)>,
    mut commands: Commands,
    q_value: Query<&SunBeanValue>,
    mut sun: ResMut<game::Sun>,
) {
    if let Ok(value) = q_value.get(entity) {
        sun.0 += value.value;
    }
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
}

fn sun_bean_tick(
    mut commands: Commands,
    mut q_value: Query<(Entity, &game::Overlay, &mut SunBeanValue)>,
    plants: Res<assets::SpritePlants>,
    config: Res<config::Config>,
    factors: Res<plants::PlantFactors>,
) {
    q_value.iter_mut().for_each(|(entity, overlay, mut value)| {
        if value.value < factors.sun_bean.max {
            value.tick(overlay.delta());
            if value.just_finished() {
                value.value += config.gamerule.sun_value.0;
                if let Some(mut commands) = commands.get_entity(entity) {
                    commands.try_insert(compn::AnimationThenDo {
                        anim: plants.sun_bean_plus.clone(),
                        ..Default::default()
                    });
                }
            }
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
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(ICEBERG_LETTUCE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: sun_bean_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .sun_bean
                .frames
                .first()
                .expect("Empty animation sun_bean")
                .clone(),
            cost: factors.sun_bean.cost,
            cooldown: factors.sun_bean.cooldown,
            hitbox: factors.sun_bean.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(SUN_BEAN, creature);
    }
}
