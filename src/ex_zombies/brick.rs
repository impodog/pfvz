use crate::prelude::*;

pub(super) struct ExZombiesBrickPlugin;

impl Plugin for ExZombiesBrickPlugin {
    fn build(&self, app: &mut App) {
        initialize(&brick_systems);
        app.add_systems(PostStartup, (init_config,));
        *brick_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_brick_zombie),
            ..Default::default()
        });
    }
}
game_conf!(systems brick_systems);
game_conf!(breaks BrickBreaks);

fn spawn_brick_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<zombies::BasicZombieWalker>,
    breaks: Res<BrickBreaks>,
) {
    let creature = map.get(&BRICK_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            zombies::BasicZombieMarker,
            creature.clone(),
            pos,
            game::Velocity::from(factors.basic.velocity),
            sprite::Animation::new(zombies.basic.clone()),
            compn::Dying::new(zombies.basic_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.basic.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.0, 0.0, 0.35, -0.1),
            ex_factors.brick.brick_box,
            sprite::Animation::new(ex_zombies.brick.clone()),
            game::Armor::new(ex_factors.brick.brick_health),
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.01),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn init_config(
    mut commands: Commands,
    ex_zombies: Res<assets::SpriteExZombies>,
    factors: Res<zombies::ZombieFactors>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(BrickBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            ex_zombies.brick.clone(),
            ex_zombies.brick_damaged.clone(),
            ex_zombies.brick_broken.clone(),
            ex_zombies.brick_destroyed.clone(),
        ],
        init: ex_factors.brick.brick_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: BRICK_ZOMBIE,
            systems: brick_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies.brick_zombie_concept.clone(),
            cost: ex_factors.brick.cost,
            cooldown: ex_factors.brick.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(creature);
    }
}
