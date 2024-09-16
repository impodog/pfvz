use crate::prelude::*;

pub(super) struct ZombiesPogoPlugin;

impl Plugin for ZombiesPogoPlugin {
    fn build(&self, app: &mut App) {
        initialize(&pogo_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (pogo_zombie_jump,).run_if(when_state!(gaming)));
        *pogo_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_pogo_zombie),
            ..Default::default()
        });
    }
}

game_conf!(systems pogo_zombie_systems);
game_conf!(walker PogoZombieWalker);

#[derive(Default, Component, Debug, Clone, Copy)]
pub enum PogoZombieStage {
    #[default]
    // This stage will immediately be switched to forward after spawning
    JumpInSitu,
    JumpForward,
    Walk,
}

fn spawn_pogo_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&POGO_ZOMBIE).unwrap();
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::default(),
        // This makes sure the walker of the zombie works normally
        game::VelocityBase::new(factors.pogo.velocity.into()),
        sprite::Animation::new(zombies.pogo_zombie.clone()),
        compn::Dying::new(zombies.pogo_zombie_dying.clone()),
        creature.hitbox,
        PogoZombieStage::default(),
        game::Gravity,
        game::Health::from(factors.pogo.self_health),
        SpriteBundle::default(),
    ));
}

fn pogo_zombie_jump(
    commands: ParallelCommands,
    mut q_pogo: Query<(
        Entity,
        &mut game::LogicPosition,
        &mut game::Velocity,
        &mut PogoZombieStage,
        &mut sprite::Animation,
    )>,
    collision: Res<game::Collision>,
    factors: Res<zombies::ZombieFactors>,
    config: Res<config::Config>,
    level: Res<level::Level>,
    q_plant: Query<&game::HitBox, With<game::Plant>>,
    walker: Res<PogoZombieWalker>,
    zombies: Res<assets::SpriteZombies>,
) {
    q_pogo
        .par_iter_mut()
        .for_each(|(entity, mut logic, mut velocity, mut stage, mut anim)| {
            let (x, _y) = level
                .config
                .layout
                .position_3d_to_coordinates(logic.base_raw());
            let disp = level.config.layout.get_disp(x);
            if velocity.z == 0.0 || logic.base_raw().z <= disp {
                let z_vel = (2.0 * config.gamerule.gravity.0 * factors.pogo.jump_height)
                    .abs()
                    .sqrt();
                match *stage {
                    PogoZombieStage::JumpInSitu => {
                        *stage = PogoZombieStage::JumpForward;
                        velocity.x = factors.pogo.jump_velocity;
                        velocity.z = z_vel;
                    }
                    PogoZombieStage::JumpForward => {
                        *stage = PogoZombieStage::JumpInSitu;
                        velocity.x = 0.0;
                        velocity.z = z_vel;
                    }
                    _ => {}
                }
            }
            match *stage {
                PogoZombieStage::Walk => {}
                _ => {
                    if let Some(coll) = collision.get(&entity) {
                        let hit = coll.iter().any(|plant| {
                            q_plant
                                .get(*plant)
                                .is_ok_and(|hitbox| hitbox.height > factors.pogo.jump_height)
                        });
                        if hit {
                            *velocity = factors.pogo.velocity.into();
                            logic.base_raw_mut().z = disp;
                            *stage = PogoZombieStage::Walk;
                            anim.replace(zombies.pogo_zombie_only.clone());
                            commands.command_scope(|mut commands| {
                                if let Some(mut commands) = commands.get_entity(entity) {
                                    commands
                                        .remove::<game::Gravity>()
                                        .insert(compn::Walker(walker.0.clone()));
                                }
                            })
                        }
                    }
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
    commands.insert_resource(PogoZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.pogo.interval),
        damage: factors.pogo.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: pogo_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .pogo_zombie
                .frames
                .first()
                .expect("empty animation pogo_zombie")
                .clone(),
            cost: factors.pogo.cost,
            cooldown: factors.pogo.cooldown,
            hitbox: factors.pogo.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(POGO_ZOMBIE, creature);
    }
}
