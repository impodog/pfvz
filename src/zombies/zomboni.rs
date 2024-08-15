use crate::prelude::*;

pub(super) struct ZombiesZomboniPlugin;

impl Plugin for ZombiesZomboniPlugin {
    fn build(&self, app: &mut App) {
        initialize(&zomboni_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (zomboni_leave_trail,).run_if(when_state!(gaming)));
        *zomboni_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_zomboni),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(walker ZomboniWalker);
game_conf!(breaks ZomboniBreaks);
game_conf!(systems zomboni_systems);

fn spawn_zomboni(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<ZomboniWalker>,
    breaks: Res<ZomboniBreaks>,
    audio: Res<Audio>,
    audio_zombies: Res<assets::AudioZombies>,
) {
    let creature = map.get(&ZOMBONI).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.zomboni.velocity),
            sprite::Animation::new(zombies.zomboni.clone()),
            compn::Dying::new(zombies.zomboni_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            compn::Breaks(breaks.0.clone()),
            ZomboniMarker,
            game::Health::from(factors.zomboni.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((compn::UnsnowParent { absolute: true },))
        .set_parent(entity);
    audio.play(audio_zombies.zomboni.random());
}

#[derive(Component)]
pub struct ZomboniMarker;

fn zomboni_leave_trail(
    commands: ParallelCommands,
    q_zomboni: Query<&game::LogicPosition, With<ZomboniMarker>>,
    level: Res<level::Level>,
    plants: Res<game::PlantLayout>,
) {
    q_zomboni.par_iter().for_each(|pos| {
        let mut pos = level.config.layout.regularize(*pos.base_raw());
        pos.z = 0.0;
        let index = level.config.layout.position_to_index(&pos);
        if let Some(plants) = plants.plants.get(index) {
            if plants.read().unwrap().is_empty() {
                commands.command_scope(|mut commands| {
                    commands.run_system_with_input(
                        plants::ice_systems.read().unwrap().unwrap().spawn,
                        game::LogicPosition::from_base(pos),
                    );
                });
            }
        }
    });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(ZomboniWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.zomboni.interval),
        damage: factors.zomboni.damage,
    })));
    commands.insert_resource(ZomboniBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            zombies.zomboni.clone(),
            zombies.zomboni_damaged.clone(),
            zombies.zomboni_destroyed.clone(),
        ],
        init: factors.zomboni.self_health.0,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: zomboni_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .zomboni
                .frames
                .first()
                .expect("empty animation zomboni")
                .clone(),
            cost: factors.zomboni.cost,
            cooldown: factors.zomboni.cooldown,
            hitbox: factors.zomboni.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(ZOMBONI, creature);
    }
}
