use crate::prelude::*;

pub(super) struct ExZombiesMirrorPlugin;

impl Plugin for ExZombiesMirrorPlugin {
    fn build(&self, app: &mut App) {
        initialize(&mirror_systems);
        app.add_systems(PostStartup, (init_config,));
        *mirror_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_mirror_zombie),
            ..Default::default()
        });
    }
}
game_conf!(systems mirror_systems);
game_conf!(breaks MirrorBreaks);

fn spawn_mirror_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<zombies::BasicZombieWalker>,
    breaks: Res<MirrorBreaks>,
) {
    let creature = map.get(&MIRROR_ZOMBIE).unwrap();
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
            game::RelativePosition::new(-0.1, 0.0, 0.1, 0.1),
            ex_factors.mirror.mirror_box,
            sprite::Animation::new(ex_zombies.mirror.clone()),
            game::Armor::new(ex_factors.mirror.mirror_health),
            compn::Breaks(breaks.0.clone()),
            compn::Mirror,
            compn::UnsnowParent { absolute: false },
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
    commands.insert_resource(MirrorBreaks(Arc::new(compn::BreaksShared {
        v: vec![ex_zombies.mirror.clone(), ex_zombies.mirror_damaged.clone()],
        init: ex_factors.mirror.mirror_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: MIRROR_ZOMBIE,
            systems: mirror_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies.mirror_zombie_concept.clone(),
            cost: ex_factors.mirror.cost,
            cooldown: ex_factors.mirror.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(creature);
    }
}
