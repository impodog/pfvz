use crate::prelude::*;

pub(super) struct ExZombiesRallyPlugin;

impl Plugin for ExZombiesRallyPlugin {
    fn build(&self, app: &mut App) {
        initialize(&rally_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(OnEnter(info::PlayStates::Gaming), (init_boosted,));
        app.add_systems(Update, (boost_zombies,).run_if(when_state!(gaming)));
        *rally_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_rally_zombie),
            ..Default::default()
        });
    }
}

game_conf!(systems rally_zombie_systems);
game_conf!(walker RallyZombieWalker);
game_conf!(breaks RallyFlagBreaks);

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct RallyBoostedZombies(pub BTreeSet<Entity>);

fn init_boosted(mut commands: Commands) {
    commands.insert_resource(RallyBoostedZombies::default());
}

#[derive(Component)]
pub struct RallyMarker;

fn spawn_rally_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<RallyZombieWalker>,
    breaks: Res<RallyFlagBreaks>,
) {
    let creature = map.get(&RALLY_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            zombies::BasicZombieMarker,
            RallyMarker,
            creature.clone(),
            pos,
            game::Velocity::from(ex_factors.rally.velocity),
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
            game::RelativePosition::new(
                -ex_factors.rally.rally_flag_box.width / 2.0,
                0.0,
                ex_factors.rally.rally_flag_box.height / 2.0,
                0.0,
            ),
            ex_factors.rally.rally_flag_box,
            sprite::Animation::new(ex_zombies.rally_flag.clone()),
            game::Armor::new(ex_factors.rally.rally_flag_health),
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.01),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn boost_zombies(
    q_rally: Query<&game::Position, With<RallyMarker>>,
    q_rally_marker: Query<(), With<RallyMarker>>,
    q_zombie: Query<
        (Entity, &game::Position, &game::HitBox),
        (With<game::Zombie>, With<game::VelocityBase>),
    >,
    mut q_velocity_base: Query<&mut game::VelocityBase>,
    mut boosted: ResMut<RallyBoostedZombies>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
) {
    let remove = Mutex::new(BTreeSet::new());
    let add = Mutex::new(BTreeSet::new());
    q_rally.par_iter().for_each(|rally_pos| {
        let range = game::PositionRange::from(ex_factors.rally.range) + *rally_pos;
        q_zombie.par_iter().for_each(|(entity, pos, hitbox)| {
            // Rally zombies can never be boosted
            if q_rally_marker.get(entity).is_ok() {
                return;
            }
            let boost = range.contains(pos, hitbox);
            if boost ^ boosted.contains(&entity) {
                if boost {
                    add.lock().unwrap().insert(entity);
                } else {
                    remove.lock().unwrap().insert(entity);
                }
            }
        });
    });
    for entity in Mutex::into_inner(remove).unwrap().into_iter() {
        if let Ok(mut velocity_base) = q_velocity_base.get_mut(entity) {
            velocity_base.divide(ex_factors.rally.boost);
            boosted.remove(&entity);
        }
    }
    for entity in Mutex::into_inner(add).unwrap().into_iter() {
        if let Ok(mut velocity_base) = q_velocity_base.get_mut(entity) {
            velocity_base.multiply(ex_factors.rally.boost);
            boosted.insert(entity);
        }
    }
}

fn init_config(
    mut commands: Commands,
    ex_zombies: Res<assets::SpriteExZombies>,
    factors: Res<zombies::ZombieFactors>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(RallyZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.rally.interval),
        damage: ex_factors.rally.damage,
    })));
    commands.insert_resource(RallyFlagBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            ex_zombies.rally_flag.clone(),
            ex_zombies.rally_flag_damaged.clone(),
        ],
        init: ex_factors.rally.rally_flag_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: RALLY_ZOMBIE,
            systems: rally_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies.rally_zombie_concept.clone(),
            cost: ex_factors.rally.cost,
            cooldown: ex_factors.rally.cooldown,
            hitbox: factors.basic.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(creature);
    }
}
