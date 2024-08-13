use crate::prelude::*;

pub struct CompnShooterPlugin;

impl Plugin for CompnShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (add_shooter_impl, shooter_work).run_if(when_state!(gaming)),
        );
    }
}

// Anything that uses this shoots projectile of their ally
#[derive(Debug, Clone)]
pub struct ShooterShared {
    pub interval: Duration,
    pub velocity: game::Velocity,
    pub proj: game::Projectile,
    pub start: Vec<game::Position>,
    pub times: usize,
    pub require_zombie: bool,
    pub after: SystemId<Entity>,
    pub callback: SystemId<Entity>,
    pub shared: Arc<game::ProjectileShared>,
}
impl Default for ShooterShared {
    fn default() -> Self {
        Self {
            interval: Default::default(),
            velocity: Default::default(),
            proj: Default::default(),
            start: vec![game::Position::default()],
            times: 1,
            require_zombie: false,
            after: compn::default::system_do_nothing.read().unwrap().unwrap(),
            callback: compn::default::system_do_nothing.read().unwrap().unwrap(),
            shared: Default::default(),
        }
    }
}
#[derive(Component, Debug, Clone, Deref)]
pub struct Shooter(pub Arc<ShooterShared>);

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct ShooterImpl {
    #[deref]
    pub timer: Timer,
}
impl From<&Shooter> for ShooterImpl {
    fn from(value: &Shooter) -> Self {
        Self {
            timer: Timer::new(value.interval, TimerMode::Repeating),
        }
    }
}

fn add_shooter_impl(mut commands: Commands, q_shooter: Query<(Entity, &Shooter), Added<Shooter>>) {
    q_shooter.iter().for_each(|(entity, shooter)| {
        commands
            .entity(entity)
            .insert((ShooterImpl::from(shooter),));
    });
}

fn shooter_work(
    commands: ParallelCommands,
    mut q_shooter: Query<(
        Entity,
        &game::Overlay,
        &Shooter,
        &mut ShooterImpl,
        &game::Position,
        &game::HitBox,
        &Transform,
    )>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_zpos: Query<&game::Position, With<game::Zombie>>,
    level: Res<level::Level>,
) {
    q_shooter.par_iter_mut().for_each(
        |(entity, overlay, shooter, mut work, pos, hitbox, transform)| {
            work.timer.tick(overlay.delta());
            if work.timer.just_finished() {
                let mut pos = (*pos).move_z(hitbox.height * -0.1);
                let range = shooter.proj.range.clone() + pos;
                if shooter.require_zombie {
                    let mut ok = false;
                    for zpos in q_zpos.iter() {
                        let zpos = level.config.layout.regularize(*zpos);
                        if range.contains(&zpos) {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        return;
                    }
                }
                for _ in 0..shooter.times {
                    for start in shooter.start.iter() {
                        let proj_entity = {
                            commands.command_scope(|mut commands| {
                                let mut commands = commands.spawn((
                                    game::LogicPosition::from_bottom(*start + pos),
                                    sprite::Animation::new(shooter.shared.anim.clone()),
                                    shooter.shared.hitbox,
                                    shooter.proj.clone(),
                                    shooter.velocity,
                                    SpriteBundle {
                                        transform: Transform::from_xyz(
                                            0.0,
                                            0.0,
                                            transform.translation.z + 0.1,
                                        ),
                                        ..Default::default()
                                    },
                                ));
                                // Determines whether the projectile is plant(default) or zombie
                                if q_zombie.get(entity).is_ok() {
                                    commands.insert(game::ZombieRelevant);
                                } else {
                                    commands.insert(game::PlantRelevant);
                                }
                                commands.id()
                            })
                        };
                        commands.command_scope(|mut commands| {
                            commands.run_system_with_input(shooter.callback, entity);
                            commands.run_system_with_input(shooter.after, proj_entity);
                        });
                    }
                    // NOTE: Do we need to make this customizable?
                    pos.x += 0.1 * shooter.velocity.x;
                }
            }
        },
    )
}
