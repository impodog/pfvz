use crate::prelude::*;

pub(super) struct PlantsDefensePlugin;

impl Plugin for PlantsDefensePlugin {
    fn build(&self, app: &mut App) {
        initialize(&wall_nut_systems);
        initialize(&tall_nut_systems);
        app.add_systems(PostStartup, (init_config,));
        *wall_nut_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_wall_nut),
            ..Default::default()
        });
        *tall_nut_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_tall_nut),
            ..Default::default()
        });
        *pumpkin_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_pumpkin),
            ..Default::default()
        });
    }
}

game_conf!(breaks WallNutBreaks);
game_conf!(systems wall_nut_systems);
game_conf!(breaks TallNutBreaks);
game_conf!(systems tall_nut_systems);
game_conf!(breaks PumpkinBreaks);
game_conf!(systems pumpkin_systems);

fn spawn_wall_nut(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    breaks: Res<WallNutBreaks>,
) {
    let creature = map.get(&WALL_NUT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.wall_nut.clone()),
        creature.hitbox,
        compn::Breaks(breaks.0.clone()),
        game::Health::from(factors.wall_nut.health),
        SpriteBundle::default(),
    ));
}

fn spawn_tall_nut(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    breaks: Res<TallNutBreaks>,
) {
    let creature = map.get(&TALL_NUT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.tall_nut.clone()),
        creature.hitbox,
        compn::Breaks(breaks.0.clone()),
        game::Health::from(factors.tall_nut.health),
        SpriteBundle::default(),
    ));
}

fn spawn_pumpkin(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    breaks: Res<PumpkinBreaks>,
) {
    let creature = map.get(&PUMPKIN).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.pumpkin.clone()),
        creature.hitbox,
        compn::Breaks(breaks.0.clone()),
        game::PlantGoBelow,
        game::Health::from(factors.pumpkin.health),
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
            id: WALL_NUT,
            systems: wall_nut_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .wall_nut
                .frames
                .first()
                .expect("Empty animation wall_nut")
                .clone(),
            cost: factors.wall_nut.cost,
            cooldown: factors.wall_nut.cooldown,
            hitbox: factors.wall_nut.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
    commands.insert_resource(TallNutBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            plants.tall_nut.clone(),
            plants.tall_nut_damaged.clone(),
            plants.tall_nut_destroyed.clone(),
        ],
        init: factors.tall_nut.health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: TALL_NUT,
            systems: tall_nut_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .tall_nut
                .frames
                .first()
                .expect("Empty animation tall_nut")
                .clone(),
            cost: factors.tall_nut.cost,
            cooldown: factors.tall_nut.cooldown,
            hitbox: factors.tall_nut.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
    commands.insert_resource(PumpkinBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            plants.pumpkin.clone(),
            plants.pumpkin_damaged.clone(),
            plants.pumpkin_destroyed.clone(),
        ],
        init: factors.pumpkin.health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: PUMPKIN,
            systems: pumpkin_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .pumpkin
                .frames
                .first()
                .expect("Empty animation pumpkin")
                .clone(),
            cost: factors.pumpkin.cost,
            cooldown: factors.pumpkin.cooldown,
            hitbox: factors.pumpkin.self_box,
            flags: level::CreatureFlags::PUMPKIN,
        }));
        map.insert(creature);
    }
}
