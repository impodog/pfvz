use crate::prelude::*;

pub(super) struct PlantsBowlingPlugin;

impl Plugin for PlantsBowlingPlugin {
    fn build(&self, app: &mut App) {
        initialize(&bowling_nut_systems);
        app.add_systems(PostStartup, (init_config,));
        *bowling_nut_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_bowling_nut),
            die: app.register_system(compn::default::die_not),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems bowling_nut_systems);

fn spawn_bowling_nut(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&BOWLING_NUT).unwrap();
    let mut velocity = factors.bowling_nut.velocity;
    velocity.y = 0.0;

    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        game::Position::default(),
        sprite::Animation::new(plants.wall_nut.clone()),
        velocity,
        creature.hitbox,
        compn::Bowling {
            damage: factors.bowling_nut.damage,
            velocity_y: factors.bowling_nut.velocity.y,
        },
        game::Health::from(factors.bowling_nut.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: bowling_nut_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants.bowling_nut_concept.clone(),
            cost: factors.bowling_nut.cost,
            cooldown: factors.bowling_nut.cooldown,
            hitbox: factors.bowling_nut.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(BOWLING_NUT, creature);
    }
}
