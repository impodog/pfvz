use crate::prelude::*;

pub(super) struct PlantsBusterPlugin;

impl Plugin for PlantsBusterPlugin {
    fn build(&self, app: &mut App) {
        initialize(&grave_buster_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(
            Update,
            (bust_grave, coffee_bean_work).run_if(when_state!(gaming)),
        );
        *grave_buster_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_grave_buster),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *coffee_bean_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_coffee_bean),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct BusterTimer(Timer);

game_conf!(systems grave_buster_systems);
game_conf!(systems coffee_bean_systems);

fn spawn_grave_buster(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&GRAVE_BUSTER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.grave_buster.clone()),
        creature.hitbox,
        BusterTimer(Timer::new(
            Duration::from_secs_f32(factors.grave_buster.interval),
            TimerMode::Once,
        )),
        game::Health::from(factors.grave_buster.health),
        SpriteBundle::default(),
    ));
}

fn bust_grave(
    mut commands: Commands,
    mut q_buster: Query<(Entity, &game::Overlay, &game::Position, &mut BusterTimer)>,
    layout: Res<game::PlantLayout>,
    level: Res<level::Level>,
) {
    q_buster
        .iter_mut()
        .for_each(|(entity, overlay, pos, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                let index = level.config.layout.position_3d_to_index(pos);
                commands.entity(entity).despawn_recursive();
                if let Some(list) = layout.plants.get(index) {
                    let list = list.read().unwrap();
                    if let Some(grave) = list.iter().rev().nth(1) {
                        commands.entity(*grave).despawn_recursive();
                    }
                }
            }
        });
}

#[derive(Component, Deref, DerefMut)]
struct CoffeeBeanTimer(Timer);

fn spawn_coffee_bean(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&COFFEE_BEAN).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.coffee_bean.clone()),
        creature.hitbox,
        CoffeeBeanTimer(Timer::from_seconds(
            factors.coffee_bean.interval,
            TimerMode::Once,
        )),
        game::Health::from(factors.coffee_bean.health),
        SpriteBundle::default(),
    ));
}

fn coffee_bean_work(
    mut action: EventWriter<game::CreatureAction>,
    mut q_coffee_bean: Query<(
        Entity,
        &game::Overlay,
        &mut CoffeeBeanTimer,
        &game::LogicPosition,
    )>,
    level: Res<level::Level>,
    plants: Res<game::PlantLayout>,
    mut q_mushroom: Query<&mut compn::Mushroom>,
) {
    q_coffee_bean
        .iter_mut()
        .for_each(|(entity, overlay, mut timer, logic)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                let index = level.config.layout.position_3d_to_index(logic.base_raw());
                if let Some(plants) = plants.plants.get(index) {
                    plants
                        .read()
                        .unwrap()
                        .iter()
                        .filter(|plant| **plant != entity)
                        .for_each(|plant| {
                            if let Ok(mut mushroom) = q_mushroom.get_mut(*plant) {
                                mushroom.0 = false;
                            }
                        })
                }
                action.send(game::CreatureAction::Die(entity));
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
            systems: grave_buster_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .grave_buster
                .frames
                .first()
                .expect("Empty animation grave_buster")
                .clone(),
            cost: factors.grave_buster.cost,
            cooldown: factors.grave_buster.cooldown,
            hitbox: factors.grave_buster.self_box,
            flags: level::CreatureFlags::GRAVE_BUSTER,
        }));
        map.insert(GRAVE_BUSTER, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: coffee_bean_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .coffee_bean
                .frames
                .first()
                .expect("Empty animation coffee_bean")
                .clone(),
            cost: factors.coffee_bean.cost,
            cooldown: factors.coffee_bean.cooldown,
            hitbox: factors.coffee_bean.self_box,
            flags: level::CreatureFlags::COFFEE_BEAN,
        }));
        map.insert(COFFEE_BEAN, creature);
    }
}
