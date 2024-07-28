use crate::prelude::*;

pub(super) struct PlantsExplodePlugin;

impl Plugin for PlantsExplodePlugin {
    fn build(&self, app: &mut App) {
        initialize(&cherry_bomb_systems);
        app.add_systems(PostStartup, (init_config,));
        *cherry_bomb_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_cherry_bomb),
            die: app.register_system(compn::default::die_not),
            damage: app.register_system(compn::default::damage),
        });
        *potato_mine_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_potato_mine),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(explode CherryBombExplode);
game_conf!(systems cherry_bomb_systems);
game_conf!(explode PotatoMineExplode);
game_conf!(systems potato_mine_systems);

fn spawn_cherry_bomb(
    In(pos): In<game::Position>,
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

fn spawn_potato_mine(
    In(mut pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    explode: Res<PotatoMineExplode>,
) {
    pos.y -= 0.2;
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
        }));
        map.insert(POTATO_MINE, creature);
    }
}
