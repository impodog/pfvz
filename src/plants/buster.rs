use crate::prelude::*;

pub(super) struct PlantsBusterPlugin;

impl Plugin for PlantsBusterPlugin {
    fn build(&self, app: &mut App) {
        initialize(&grave_buster_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (bust_grave,));
        *grave_buster_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_grave_buster),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct BusterTimer(Timer);

game_conf!(systems grave_buster_systems);

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
                let index = level.config.layout.position_to_index(pos);
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
}
