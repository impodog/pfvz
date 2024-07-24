use crate::prelude::*;

pub(super) struct PlantsDefensePlugin;

impl Plugin for PlantsDefensePlugin {
    fn build(&self, app: &mut App) {
        initialize(&wall_nut_systems);
        app.add_systems(PostStartup, (init_config,));
        *wall_nut_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_wall_nut),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(breaks WallNutBreaks);
game_conf!(systems wall_nut_systems);

fn spawn_wall_nut(
    In(pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    map: Res<game::CreatureMap>,
    breaks: Res<WallNutBreaks>,
) {
    let creature = map.get(&WALL_NUT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(creature.anim.clone()),
        creature.hitbox,
        compn::Breaks(breaks.0.clone()),
        game::Health::from(factors.wall_nut.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(WallNutBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            plants.wall_nut.clone(),
            plants.wall_nut_damaged.clone(),
            plants.wall_nut_destroyed.clone(),
        ],
        init: factors.wall_nut.health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: wall_nut_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            anim: plants.wall_nut.clone(),
            cost: factors.wall_nut.cost,
            cooldown: factors.wall_nut.cooldown,
            hitbox: factors.wall_nut.self_box,
        }));
        map.insert(WALL_NUT, creature);
    }
}
