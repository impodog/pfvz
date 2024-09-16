use crate::prelude::*;

pub(super) struct ZombiesDancePlugin;

impl Plugin for ZombiesDancePlugin {
    fn build(&self, app: &mut App) {
        initialize(&dancing_zombie_systems);
        initialize(&backup_dancer_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(
            Update,
            (dancing_finish_back, dancing_try_spawn).run_if(when_state!(gaming)),
        );
        *dancing_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_dancing_zombie),
            ..Default::default()
        });
        *backup_dancer_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_backup_dancer),
            ..Default::default()
        });
        *summon_backup_system.write().unwrap() = Some(app.register_system(summon_backup));
    }
}

game_conf!(systems dancing_zombie_systems);
game_conf!(systems backup_dancer_systems);
game_conf!(system summon_backup_system, (Entity, usize, game::LogicPosition));
game_conf!(walker DancingWalker);

const DIRECTION: [(f32, f32); 4] = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct DancingBackTimer(pub Timer);

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct DancingSummonTimer {
    #[deref]
    pub timer: Timer,
    pub backup: [Option<Entity>; 4],
}

fn spawn_dancing_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<DancingWalker>,
) {
    let creature = map.get(&DANCING_ZOMBIE).unwrap();
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::from(factors.dancing.velocity_back),
        game::VelocityBase::new(factors.dancing.velocity.into()),
        sprite::Animation::new(zombies.dancing_zombie_back.clone()),
        compn::Dying::new(zombies.dancing_zombie_dying.clone()),
        creature.hitbox,
        DancingBackTimer(Timer::new(
            Duration::from_secs_f32(factors.dancing.back_time),
            TimerMode::Once,
        )),
        DancingSummonTimer {
            timer: Timer::new(
                Duration::from_secs_f32(factors.dancing.spawn_interval),
                TimerMode::Repeating,
            ),
            ..Default::default()
        },
        compn::Walker(walker.0.clone()),
        game::Health::from(factors.dancing.self_health),
        SpriteBundle::default(),
    ));
}

fn dancing_finish_back(
    commands: ParallelCommands,
    mut q_dancing: Query<(
        Entity,
        &game::Overlay,
        &mut DancingBackTimer,
        &mut game::Velocity,
        &mut compn::WalkerImpl,
        &mut sprite::Animation,
    )>,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_dancing.par_iter_mut().for_each(
        |(entity, overlay, mut timer, mut velocity, mut walker_impl, mut anim)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                *velocity = game::Velocity::from(factors.dancing.velocity);
                walker_impl.target = None;
                anim.replace(zombies.dancing_zombie.clone());
                commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<DancingBackTimer>();
                });
            }
        },
    );
}

fn summon_backup(
    In((entity, index, pos)): In<(Entity, usize, game::LogicPosition)>,
    mut commands: Commands,
    mut q_timer: Query<&mut DancingSummonTimer>,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<DancingWalker>,
) {
    let backup = spawn_backup_dancer_with(In(pos), zombies, &mut commands, factors, map, walker);
    if let Ok(mut timer) = q_timer.get_mut(entity) {
        if let Some(backup_ref) = timer.backup.get_mut(index) {
            *backup_ref = Some(backup);
        }
    }
}

fn dancing_try_spawn(
    commands: ParallelCommands,
    mut q_dancing: Query<(
        Entity,
        &game::LogicPosition,
        &game::Overlay,
        &mut DancingSummonTimer,
    )>,
    zombies: Res<assets::SpriteZombies>,
    level: Res<level::Level>,
) {
    let is_ok = |entity: Option<&Entity>| {
        entity.is_some_and(|entity| {
            commands.command_scope(|mut commands| commands.get_entity(*entity).is_some())
        })
    };
    q_dancing
        .par_iter_mut()
        .for_each(|(entity, pos, overlay, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                let direction = DIRECTION;
                for (index, (x, y)) in direction.into_iter().enumerate() {
                    if !is_ok(timer.backup.get(index).and_then(|inner| inner.as_ref())) {
                        let pos = pos.bottom().move_by(x, y);
                        if level
                            .config
                            .layout
                            .position_2d_to_coordinates_checked(&pos)
                            .is_some()
                        {
                            commands.command_scope(|mut commands| {
                                commands.entity(entity).try_insert(compn::AnimationThenDo {
                                    anim: zombies.dancing_zombie_summon.clone(),
                                    ..Default::default()
                                });
                                commands.run_system_with_input(
                                    summon_backup_system.read().unwrap().unwrap(),
                                    (entity, index, game::LogicPosition::from_bottom(pos)),
                                );
                            });
                        }
                    }
                }
            }
        });
}

fn spawn_backup_dancer(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<DancingWalker>,
) {
    spawn_backup_dancer_with(In(pos), zombies, &mut commands, factors, map, walker);
}

fn spawn_backup_dancer_with(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    commands: &mut Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<DancingWalker>,
) -> Entity {
    let creature = map.get(&BACKUP_DANCER).unwrap();
    commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.dancing.velocity),
            sprite::Animation::new(zombies.backup_dancer.clone()),
            compn::Dying::new(zombies.backup_dancer_dying.clone()),
            creature.hitbox,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.dancing.backup_health),
            SpriteBundle::default(),
        ))
        .id()
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(DancingWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.dancing.interval),
        damage: factors.all_star.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: dancing_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .dancing_zombie
                .frames
                .first()
                .expect("Empty animation dancing_zombie")
                .clone(),
            cost: factors.dancing.cost,
            cooldown: factors.dancing.cooldown,
            hitbox: factors.dancing.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(DANCING_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: backup_dancer_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .backup_dancer
                .frames
                .first()
                .expect("Empty animation backup_dancer")
                .clone(),
            cost: factors.dancing.cost,
            cooldown: factors.dancing.cooldown,
            hitbox: factors.dancing.backup_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(BACKUP_DANCER, creature);
    }
}
