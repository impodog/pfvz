use crate::prelude::*;

pub(super) struct ZombiesGargantuarPlugin;

impl Plugin for ZombiesGargantuarPlugin {
    fn build(&self, app: &mut App) {
        initialize(&gargantuar_systems);
        initialize(&gargantuar_smash_system);
        initialize(&imp_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(
            Update,
            (
                gargantuar_begin_smash,
                gargantuar_add_bandaid,
                gargantuar_throw_imp,
                imp_hit_ground,
            )
                .run_if(when_state!(gaming)),
        );
        *gargantuar_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_gargantuar),
            ..Default::default()
        });
        *gargantuar_smash_system.write().unwrap() = Some(app.register_system(gargantuar_smash));
        *imp_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_imp),
            ..Default::default()
        });
    }
}

game_conf!(systems gargantuar_systems);
game_conf!(system gargantuar_smash_system, Entity);
game_conf!(systems imp_systems);
game_conf!(walker ImpWalker);

fn spawn_gargantuar(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<ImpWalker>,
) {
    let creature = map.get(&GARGANTUAR).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.gargantuar.velocity),
            sprite::Animation::new(zombies.gargantuar.clone()),
            creature.hitbox,
            GargantuarImpl {
                init: factors.gargantuar.self_health.0,
                ..Default::default()
            },
            game::Health::from(factors.gargantuar.self_health),
            SpriteBundle::default(),
        ))
        .id();
    let imp = spawn_imp_with(
        In(game::LogicPosition::from_bottom(game::Position::new(
            0.5,
            0.0,
            factors.gargantuar.self_box.height / 2.0 - factors.imp.self_box.height / 2.0,
            0.0,
        ))),
        &zombies,
        &mut commands,
        &factors,
        &map,
        &walker,
    );
    commands
        .entity(imp)
        .set_parent(entity)
        .remove::<game::Zombie>()
        .insert(game::Velocity::default())
        .insert(game::LayerDisp(-0.01));
}

#[derive(Component)]
pub struct ImpMarker;

fn spawn_imp_with(
    In(pos): In<game::LogicPosition>,
    zombies: &Res<assets::SpriteZombies>,
    commands: &mut Commands,
    factors: &Res<zombies::ZombieFactors>,
    map: &Res<game::CreatureMap>,
    walker: &Res<ImpWalker>,
) -> Entity {
    let creature = map.get(&IMP).unwrap();
    let velocity = game::Velocity::from(factors.imp.velocity);
    commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            velocity,
            game::VelocityBase(velocity),
            sprite::Animation::new(zombies.imp.clone()),
            compn::Dying::new(zombies.imp_dying.clone()),
            creature.hitbox,
            ImpMarker,
            compn::Walker(walker.0.clone()),
            game::Health::from(factors.imp.self_health),
            SpriteBundle::default(),
        ))
        .id()
}

fn spawn_imp(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<ImpWalker>,
) {
    spawn_imp_with(In(pos), &zombies, &mut commands, &factors, &map, &walker);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum BandaidStatus {
    #[default]
    None,
    One,
    Two,
}

#[derive(Component, Default)]
pub struct GargantuarImpl {
    target: Option<Entity>,
    init: u32,
    bandaid: BandaidStatus,
    imp_thrown: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct GargantuarInterval(pub Timer);

impl Default for GargantuarInterval {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once))
    }
}

fn gargantuar_smash(
    In(entity): In<Entity>,
    mut commands: Commands,
    mut q_garg: Query<(
        &game::VelocityBase,
        &mut GargantuarImpl,
        &mut game::Velocity,
    )>,
    mut action: EventWriter<game::CreatureAction>,
    factors: Res<zombies::ZombieFactors>,
) {
    if let Ok((velocity_base, mut garg, mut velocity)) = q_garg.get_mut(entity) {
        if let Some(target) = std::mem::take(&mut garg.target) {
            action.send(game::CreatureAction::Damage(
                target,
                factors.gargantuar.damage,
            ));
            *velocity = **velocity_base;
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.try_insert(GargantuarInterval::default());
            }
        }
    }
}

fn gargantuar_begin_smash(
    commands: ParallelCommands,
    mut q_garg: Query<(
        Entity,
        &game::Overlay,
        &mut GargantuarImpl,
        &mut game::Velocity,
        Option<&mut GargantuarInterval>,
    )>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_creature: Query<(), With<game::Creature>>,
    collision: Res<game::Collision>,
    zombies: Res<assets::SpriteZombies>,
) {
    q_garg
        .par_iter_mut()
        .for_each(|(entity, overlay, mut garg, mut velocity, interval)| {
            let ok = if let Some(mut interval) = interval {
                interval.tick(overlay.delta());
                interval.finished()
            } else {
                true
            };
            if ok && garg.target.is_none() {
                if let Some(coll) = collision.get(&entity) {
                    let is_zombie = q_zombie.get(entity).is_ok();
                    let target = coll
                        .iter()
                        .find(|target| {
                            q_creature.get(**target).is_ok()
                                && (q_zombie.get(**target).is_ok() ^ is_zombie)
                        })
                        .copied();
                    if target.is_some() {
                        garg.target = target;
                        *velocity = game::Velocity::default();

                        commands.command_scope(|mut commands| {
                            if let Some(mut commands) = commands.get_entity(entity) {
                                commands.try_insert(compn::AnimationThenDo {
                                    anim: zombies.gargantuar_smash.clone(),
                                    work: gargantuar_smash_system.read().unwrap().unwrap(),
                                });
                            }
                        })
                    }
                }
            }
        });
}

#[derive(Component)]
pub struct BandaidMarker;

fn gargantuar_add_bandaid(
    commands: ParallelCommands,
    mut q_garg: Query<(Entity, &mut GargantuarImpl, &game::Health, &game::HitBox)>,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_garg
        .par_iter_mut()
        .for_each(|(entity, mut garg, health, hitbox)| {
            let segment = garg.init / 3;
            let health = health.value();
            let next_status = if health <= segment {
                BandaidStatus::Two
            } else if health <= segment + segment {
                BandaidStatus::One
            } else {
                BandaidStatus::None
            };
            let diff = u8::from(next_status).saturating_sub(garg.bandaid.into());
            (0..diff).for_each(|_| {
                commands.command_scope(|mut commands| {
                    let x = rand::thread_rng().gen_range(-hitbox.width / 2.0..hitbox.width / 2.0);
                    let z = rand::thread_rng().gen_range(-hitbox.height / 2.0..hitbox.height / 2.0);
                    let r =
                        rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI);
                    let child = commands
                        .spawn((
                            game::Position::default(),
                            game::RelativePosition::new(x, 0.0, z, r),
                            factors.gargantuar.bandaid_box,
                            sprite::Animation::new(zombies.bandaid.clone()),
                            game::LayerDisp(0.01),
                            SpriteBundle::default(),
                        ))
                        .id();
                    if let Some(mut commands) = commands.get_entity(entity) {
                        commands.add_child(child);
                    }
                });
            });
            if diff > 0 {
                garg.bandaid = next_status;
            }
        });
}

fn gargantuar_throw_imp(
    commands: ParallelCommands,
    mut q_garg: Query<(
        Entity,
        &game::LogicPosition,
        &Children,
        &mut GargantuarImpl,
        &game::Health,
    )>,
    q_creature: Query<(), With<game::Creature>>,
    q_logic: Query<&game::LogicPosition>,
    factors: Res<zombies::ZombieFactors>,
    level: Res<level::Level>,
    config: Res<config::Config>,
) {
    q_garg
        .par_iter_mut()
        .for_each(|(entity, logic, children, mut garg, health)| {
            if !garg.imp_thrown && health.value() <= garg.init / 2 {
                garg.imp_thrown = true;
                children
                    .iter()
                    .filter(|child| q_creature.get(**child).is_ok())
                    .for_each(|imp| {
                        if let Ok(imp_logic) = q_logic.get(*imp) {
                            let base = *imp_logic.base_raw() + *logic.base_raw();
                            let target_x = (base.x - factors.gargantuar.throw_distance)
                                .max(-level.config.layout.half_size_f32().0 + 0.5);
                            let target = game::Position::new(target_x, base.y, base.z, base.r);
                            let target = level.config.layout.regularize_xyzr(&target);
                            let velocity = compn::TargetCalculator {
                                diff: target - base,
                                target_vel: game::Velocity::default(),
                                x: factors.gargantuar.throw_velocity.0,
                                r: 0.0,
                                g: config.gamerule.gravity.0,
                            }
                            .calc();
                            commands.command_scope(|mut commands| {
                                if let Some(mut commands) = commands.get_entity(entity) {
                                    commands.remove_children(&[*imp]);
                                }
                                if let Some(mut commands) = commands.get_entity(*imp) {
                                    commands
                                        .try_insert(game::Zombie)
                                        .try_insert(game::LogicPosition::from_base(base))
                                        .try_insert(game::LayerDisp(0.0))
                                        .try_insert(game::Gravity)
                                        .try_insert(velocity);
                                }
                            })
                        }
                    });
            }
        });
}

fn imp_hit_ground(
    commands: ParallelCommands,
    mut q_imp: Query<
        (Entity, &mut game::LogicPosition, &mut game::Velocity),
        (With<game::Gravity>, With<ImpMarker>),
    >,
    level: Res<level::Level>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_imp
        .par_iter_mut()
        .for_each(|(entity, mut logic, mut velocity)| {
            let target_z = level.config.layout.get_disp_of(logic.base_raw());
            if logic.base_raw().z <= target_z {
                logic.base_raw_mut().z = target_z;
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(entity) {
                        commands.remove::<game::Gravity>();
                        *velocity = game::Velocity::from(factors.imp.velocity);
                    }
                });
            }
        });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: gargantuar_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .gargantuar
                .frames
                .first()
                .expect("empty animation gargantuar")
                .clone(),
            cost: factors.gargantuar.cost,
            cooldown: factors.gargantuar.cooldown,
            hitbox: factors.gargantuar.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(GARGANTUAR, creature);
    }
    commands.insert_resource(ImpWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.imp.interval),
        damage: factors.imp.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: imp_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .imp
                .frames
                .first()
                .expect("empty animation imp")
                .clone(),
            cost: factors.imp.cost,
            cooldown: factors.imp.cooldown,
            hitbox: factors.imp.self_box,
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(IMP, creature);
    }
}
