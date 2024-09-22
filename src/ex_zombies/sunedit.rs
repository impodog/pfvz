use crate::prelude::*;

pub(super) struct ExZombiesSuneditPlugin;

impl Plugin for ExZombiesSuneditPlugin {
    fn build(&self, app: &mut App) {
        initialize(&sunedit_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        *sunedit_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sunedit_zombie),
            ..Default::default()
        });
    }
}

game_conf!(walker SuneditZombieWalker);
game_conf!(walker SuneditZombieRageWalker);
game_conf!(dog SuneditDog);
game_conf!(breaks SuneditBreaks);
game_conf!(systems sunedit_zombie_systems);

fn spawn_sunedit_zombie(
    In(pos): In<game::LogicPosition>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<SuneditZombieWalker>,
    breaks: Res<SuneditBreaks>,
    dog: Res<SuneditDog>,
) {
    let creature = map.get(&SUNDAY_EDITION_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(ex_factors.sunedit.velocity),
            sprite::Animation::new(ex_zombies.sunday_edition_zombie.clone()),
            compn::Dying::new(ex_zombies.sunday_edition_zombie_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(ex_factors.sunedit.self_health),
            SpriteBundle::default(),
        ))
        .id();
    let sunedit = commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(-0.1, 0.0, 0.0, -0.1),
            ex_factors.sunedit.sunedit_box,
            sprite::Animation::new(ex_zombies.sunday_edition.clone()),
            game::Armor::new(ex_factors.sunedit.sunedit_health),
            compn::Breaks(breaks.0.clone()),
            compn::UnsnowParent { absolute: false },
            game::LayerDisp(0.1),
            SpriteBundle::default(),
        ))
        .set_parent(entity)
        .id();
    commands.entity(entity).try_insert(compn::Dog {
        shared: dog.0.clone(),
        owner: sunedit,
    });
}

fn init_config(
    mut commands: Commands,
    ex_zombies: Res<assets::SpriteExZombies>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(SuneditZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.sunedit.interval),
        damage: ex_factors.sunedit.damage,
    })));
    let rage_walker = Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.sunedit.rage_interval),
        damage: ex_factors.sunedit.damage,
    });
    commands.insert_resource(SuneditZombieRageWalker(rage_walker.clone()));
    commands.insert_resource(SuneditBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            ex_zombies.sunday_edition.clone(),
            ex_zombies.sunday_edition_damaged.clone(),
            ex_zombies.sunday_edition_destroyed.clone(),
        ],
        init: ex_factors.sunedit.sunedit_health,
    })));
    commands.insert_resource(SuneditDog(Arc::new(compn::DogShared {
        angry_velocity: ex_factors.sunedit.rage_velocity.into(),
        angry_walker: Some(rage_walker),
        ..Default::default()
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: SUNDAY_EDITION_ZOMBIE,
            systems: sunedit_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies
                .sunday_edition_zombie
                .frames
                .first()
                .expect("empty animation sunday_edition_zombie")
                .clone(),
            cost: ex_factors.sunedit.cost,
            cooldown: ex_factors.sunedit.cooldown,
            hitbox: ex_factors.sunedit.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
}
