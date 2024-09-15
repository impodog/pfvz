use crate::prelude::*;

pub(super) struct PlantsExplodePlugin;

impl Plugin for PlantsExplodePlugin {
    fn build(&self, app: &mut App) {
        initialize(&cherry_bomb_systems);
        app.add_systems(PostStartup, (init_config,));
        *cherry_bomb_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_cherry_bomb),
            ..Default::default()
        });
        *potato_mine_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_potato_mine),
            ..Default::default()
        });
        *jalapeno_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_jalapeno),
            ..Default::default()
        });
    }
}

game_conf!(explode CherryBombExplode);
game_conf!(systems cherry_bomb_systems);
game_conf!(explode PotatoMineExplode);
game_conf!(systems potato_mine_systems);
game_conf!(explode JalapenoExplode);
game_conf!(systems jalapeno_systems);

fn spawn_cherry_bomb(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    explode: Res<CherryBombExplode>,
) {
    let creature = map.get(&CHERRY_BOMB).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.cherry_bomb.clone()),
        creature.hitbox,
        compn::Explode(explode.0.clone()),
        compn::CherryBombTimer(Timer::new(
            Duration::from_secs_f32(factors.cherry_bomb.countdown),
            TimerMode::Once,
        )),
        game::Health::from(factors.cherry_bomb.health),
        SpriteBundle::default(),
    ));
}

fn spawn_jalapeno(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    explode: Res<JalapenoExplode>,
) {
    let creature = map.get(&JALAPENO).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.jalapeno.clone()),
        creature.hitbox,
        compn::Explode(explode.0.clone()),
        compn::CherryBombTimer(Timer::new(
            Duration::from_secs_f32(factors.jalapeno.countdown),
            TimerMode::Once,
        )),
        game::Health::from(factors.jalapeno.health),
        SpriteBundle::default(),
    ));
}

fn spawn_potato_mine(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    explode: Res<PotatoMineExplode>,
) {
    let creature = map.get(&POTATO_MINE).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.potato_mine_preparing.clone()),
        creature.hitbox,
        compn::Explode(explode.0.clone()),
        compn::PotatoMineTimer {
            timer: Timer::new(
                Duration::from_secs_f32(factors.potato_mine.prepare),
                TimerMode::Once,
            ),
            prepared: plants.potato_mine.clone(),
        },
        game::Health::from(factors.potato_mine.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(CherryBombExplode(Arc::new(compn::ExplodeShared {
        anim: plants.boom.clone(),
        animation_time: Duration::from_secs_f32(factors.cherry_bomb.animation_time),
        hitbox: factors.cherry_bomb.boom_box,
        damage: factors.cherry_bomb.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: cherry_bomb_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .cherry_bomb
                .frames
                .first()
                .expect("Empty animation cherry_bomb")
                .clone(),
            cost: factors.cherry_bomb.cost,
            cooldown: factors.cherry_bomb.cooldown,
            hitbox: factors.cherry_bomb.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(CHERRY_BOMB, creature);
    }
    commands.insert_resource(PotatoMineExplode(Arc::new(compn::ExplodeShared {
        anim: plants.spudow.clone(),
        animation_time: Duration::from_secs_f32(factors.potato_mine.animation_time),
        hitbox: factors.potato_mine.boom_box,
        damage: factors.potato_mine.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: potato_mine_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .potato_mine
                .frames
                .first()
                .expect("Empty animation potato_mine")
                .clone(),
            cost: factors.potato_mine.cost,
            cooldown: factors.potato_mine.cooldown,
            hitbox: factors.potato_mine.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(POTATO_MINE, creature);
    }
    commands.insert_resource(JalapenoExplode(Arc::new(compn::ExplodeShared {
        anim: plants.boom.clone(),
        animation_time: Duration::from_secs_f32(factors.jalapeno.animation_time),
        hitbox: factors.jalapeno.boom_box,
        damage: factors.jalapeno.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: jalapeno_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .jalapeno
                .frames
                .first()
                .expect("Empty animation jalapeno")
                .clone(),
            cost: factors.jalapeno.cost,
            cooldown: factors.jalapeno.cooldown,
            hitbox: factors.jalapeno.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(JALAPENO, creature);
    }
}
