use crate::prelude::*;

pub(super) struct PlantsGarlicPlugin;

impl Plugin for PlantsGarlicPlugin {
    fn build(&self, app: &mut App) {
        initialize(&garlic_systems);
        app.add_systems(PostStartup, (init_config,));
        *garlic_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_garlic),
            damage: app.register_system(garlic_divert),
            die: app.register_system(compn::default::die),
        });
    }
}

game_conf!(systems garlic_systems);
game_conf!(breaks GarlicBreaks);

fn spawn_garlic(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    breaks: Res<GarlicBreaks>,
) {
    let creature = map.get(&GARLIC).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.garlic.clone()),
        creature.hitbox,
        compn::Breaks(breaks.0.clone()),
        game::Health::from(factors.garlic.health),
        SpriteBundle::default(),
    ));
}

fn garlic_divert(
    In((entity, damage)): In<(Entity, u32)>,
    mut commands: Commands,
    mut q_health: Query<&mut game::Health>,
    collision: Res<game::Collision>,
    q_zombie: Query<&game::Position, With<game::Zombie>>,
    factors: Res<plants::PlantFactors>,
    level: Res<level::Level>,
) {
    if let Some(coll) = collision.get(&entity) {
        for zombie in coll.iter() {
            if let Ok(pos) = q_zombie.get(*zombie) {
                let target = {
                    let mut diff = [1.0, -1.0]
                        .choose(&mut rand::thread_rng())
                        .copied()
                        .unwrap();
                    let next_pos = pos.move_by(0.0, diff);
                    if level
                        .config
                        .layout
                        .position_3d_to_coordinates_checked(&next_pos)
                        .is_none()
                    {
                        diff = -diff;
                    }
                    pos.y + diff
                };
                if let Some(mut commands) = commands.get_entity(*zombie) {
                    commands.try_insert(compn::Divert::new(target, factors.garlic.velocity.0));
                }
            }
        }
    }
    if let Ok(mut health) = q_health.get_mut(entity) {
        health.decr(damage);
    }
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(GarlicBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            plants.garlic.clone(),
            plants.garlic_damaged.clone(),
            plants.garlic_destroyed.clone(),
        ],
        init: factors.garlic.health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: garlic_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .garlic
                .frames
                .first()
                .expect("Empty animation garlic")
                .clone(),
            cost: factors.garlic.cost,
            cooldown: factors.garlic.cooldown,
            hitbox: factors.garlic.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(GARLIC, creature);
    }
}
