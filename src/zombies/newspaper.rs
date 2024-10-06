use crate::prelude::*;

pub(super) struct ZombiesNewspaperPlugin;

impl Plugin for ZombiesNewspaperPlugin {
    fn build(&self, app: &mut App) {
        initialize(&newspaper_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        *newspaper_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_newspaper_zombie),
            ..Default::default()
        });
    }
}

game_conf!(walker NewspaperZombieWalker);
game_conf!(walker NewspaperZombieRageWalker);
game_conf!(dog NewspaperDog);
game_conf!(breaks NewspaperBreaks);
game_conf!(systems newspaper_zombie_systems);

fn spawn_newspaper_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<NewspaperZombieWalker>,
    breaks: Res<NewspaperBreaks>,
    dog: Res<NewspaperDog>,
) {
    let creature = map.get(&NEWSPAPER_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.newspaper.velocity),
            sprite::Animation::new(zombies.newspaper_zombie.clone()),
            compn::Dying::new(zombies.newspaper_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.newspaper.self_health),
            SpriteBundle::default(),
        ))
        .id();
    let newspaper = commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(-0.03, 0.0, 0.0, -0.1),
            factors.newspaper.newspaper_box,
            sprite::Animation::new(zombies.newspaper.clone()),
            game::Armor::new(factors.newspaper.newspaper_health),
            compn::Breaks(breaks.0.clone()),
            compn::UnsnowParent { absolute: false },
            game::LayerDisp(0.1),
            SpriteBundle::default(),
        ))
        .set_parent(entity)
        .id();
    commands.entity(entity).try_insert(compn::Dog {
        shared: dog.0.clone(),
        owner: newspaper,
    });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(NewspaperZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.newspaper.interval),
        damage: factors.newspaper.damage,
    })));
    let rage_walker = Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.newspaper.rage_interval),
        damage: factors.newspaper.damage,
    });
    commands.insert_resource(NewspaperZombieRageWalker(rage_walker.clone()));
    commands.insert_resource(NewspaperBreaks(Arc::new(compn::BreaksShared {
        v: vec![zombies.newspaper.clone(), zombies.newspaper_broken.clone()],
        init: factors.newspaper.newspaper_health,
    })));
    commands.insert_resource(NewspaperDog(Arc::new(compn::DogShared {
        angry_velocity: factors.newspaper.rage_velocity.into(),
        angry_walker: Some(rage_walker),
        ..Default::default()
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: NEWSPAPER_ZOMBIE,
            systems: newspaper_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .newspaper_zombie
                .frames
                .first()
                .expect("empty animation newspaper_zombie")
                .clone(),
            cost: factors.newspaper.cost,
            cooldown: factors.newspaper.cooldown,
            hitbox: factors.newspaper.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
}
