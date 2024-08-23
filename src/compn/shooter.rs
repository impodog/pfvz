use crate::prelude::*;

pub struct CompnShooterPlugin;

impl Plugin for CompnShooterPlugin {
    fn build(&self, app: &mut App) {
        initialize(&shooter_sound);
        app.add_systems(PreStartup, (init_shooter,));
        app.add_systems(
            PreUpdate,
            (add_shooter_impl, shooter_work).run_if(when_state!(gaming)),
        );
    }
}

lazy_static! {
    static ref shooter_sound: RwLock<Option<assets::AudioList>> = RwLock::new(None);
}

fn init_shooter(server: Res<AssetServer>) {
    *shooter_sound.write().unwrap() =
        Some(assets::AudioList::load(&server, "audio/plants/shooter"));
}
#[derive(Debug, Clone, Copy)]
pub enum RequireZombie {
    No,
    InRange,
    RayCast,
}
#[derive(Debug, Clone, Copy)]
pub enum ShooterVelocity {
    Classic(game::Velocity),
    Lobbed { x: f32, r: f32 },
}
impl Default for ShooterVelocity {
    fn default() -> Self {
        Self::Classic(Default::default())
    }
}

// Anything that uses this shoots projectile of their ally
#[derive(Debug, Clone)]
pub struct ShooterShared {
    pub interval: Duration,
    pub velocity: ShooterVelocity,
    pub proj: game::Projectile,
    pub start: Vec<(game::Position, f32)>,
    pub times: usize,
    pub require_zombie: RequireZombie,
    pub after: SystemId<Entity>,
    pub callback: SystemId<Entity>,
    pub shared: Arc<game::ProjectileShared>,
    pub audio: assets::AudioList,
}
impl Default for ShooterShared {
    fn default() -> Self {
        Self {
            interval: Default::default(),
            velocity: Default::default(),
            proj: Default::default(),
            start: vec![Default::default()],
            times: 1,
            require_zombie: RequireZombie::No,
            after: compn::default::system_do_nothing.read().unwrap().unwrap(),
            callback: compn::default::system_do_nothing.read().unwrap().unwrap(),
            shared: Default::default(),
            audio: shooter_sound
                .read()
                .unwrap()
                .clone()
                .expect("shooter_sound is not initialized"),
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

trait RayIntersectsAabb {
    fn intersects(&self, aabb: &bevy::math::bounding::Aabb2d) -> bool;
}

impl RayIntersectsAabb for Ray2d {
    fn intersects(&self, aabb: &bevy::math::bounding::Aabb2d) -> bool {
        let inv_dir = Vec2::new(1.0 / self.direction.x, 1.0 / self.direction.y);

        let t1 = (aabb.min.x - self.origin.x) * inv_dir.x;
        let t2 = (aabb.max.x - self.origin.x) * inv_dir.x;
        let t3 = (aabb.min.y - self.origin.y) * inv_dir.y;
        let t4 = (aabb.max.y - self.origin.y) * inv_dir.y;

        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));

        tmax >= tmin && tmax >= 0.0
    }
}

/// Calculates the initial velocity needed to hit an object at distance diff
/// with gravity and the x orthogonal velocity of the initial specified
struct TargetCalculator {
    diff: game::Position,
    x: f32,
    r: f32,
    g: f32,
}
impl TargetCalculator {
    fn calc(self) -> game::Velocity {
        let t = self.diff.x / self.x;
        let z_velocity = (self.diff.z + 0.5 * self.g * t * t) / t;
        game::Velocity {
            x: self.x,
            y: self.diff.y / t,
            z: z_velocity,
            r: self.r,
        }
    }
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
    )>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_zpos: Query<(&game::Position, &game::HitBox, &game::Velocity), With<game::Zombie>>,
    time: Res<config::FrameTime>,
    config: Res<config::Config>,
    audio: Res<Audio>,
) {
    q_shooter
        .par_iter_mut()
        .for_each(|(entity, overlay, shooter, mut work, pos, hitbox)| {
            work.timer.tick(overlay.delta());
            if work.timer.just_finished() {
                let mut pos = (*pos).move_z(hitbox.height * -0.05);

                // Test if the shooter should work
                let range = shooter.proj.range.clone() + pos;
                let ok = match shooter.require_zombie {
                    RequireZombie::No => true,
                    RequireZombie::InRange => {
                        q_zpos
                            .iter()
                            .any(|(zombie_pos, zombie_hitbox, _zombie_velocity)| {
                                range.contains(zombie_pos, zombie_hitbox)
                            })
                    }
                    RequireZombie::RayCast => shooter.start.iter().any(|(start, angle)| {
                        use bevy::math::{bounding::Aabb2d, Ray2d};
                        let start = *start + pos;
                        let ray = Ray2d::new(
                            Vec2::new(start.x, start.y),
                            Vec2::new(angle.cos(), angle.sin()),
                        );
                        q_zpos
                            .iter()
                            .any(|(zombie_pos, zombie_hitbox, _zombie_velocity)| {
                                let aabb = Aabb2d::new(
                                    Vec2::new(zombie_pos.x, zombie_pos.y),
                                    Vec2::new(zombie_hitbox.width, 0.8),
                                );
                                ray.intersects(&aabb)
                            })
                    }),
                };
                if !ok {
                    return;
                }

                // Calculate velocity

                let velocity = match shooter.velocity {
                    ShooterVelocity::Classic(velocity) => Some(velocity),
                    ShooterVelocity::Lobbed { x, r } => {
                        let target = q_zpos
                            .iter()
                            .filter_map(|(zombie_pos, zombie_hitbox, zombie_velocity)| {
                                if range.contains(zombie_pos, zombie_hitbox) {
                                    Some((*zombie_pos, *zombie_velocity))
                                } else {
                                    None
                                }
                            })
                            .max_by(|(lhs, _), (rhs, _)| {
                                // Reversed comparing makes sure the first zombie is hit
                                rhs.x.partial_cmp(&lhs.x).unwrap()
                            });
                        target.map(|(target, velocity)| {
                            let mut diff = target - pos;
                            diff.x -= time.diff() * velocity.x;
                            diff.y -= time.diff() * velocity.y;
                            diff.z -= time.diff() * velocity.z;
                            TargetCalculator {
                                diff,
                                x,
                                r,
                                g: config.gamerule.gravity.0,
                            }
                            .calc()
                        })
                    }
                };
                if let Some(velocity) = velocity {
                    for _ in 0..shooter.times {
                        for (start, angle) in shooter.start.iter() {
                            let proj_entity = {
                                commands.command_scope(|mut commands| {
                                    let angle = velocity.y.atan2(velocity.x) + angle;
                                    let len = velocity.x.hypot(velocity.y);
                                    let velocity = game::Velocity {
                                        x: len * angle.cos(),
                                        y: len * angle.sin(),
                                        z: velocity.z,
                                        r: velocity.r,
                                    };
                                    let mut commands = commands.spawn((
                                        game::LogicPosition::from_bottom(*start + pos),
                                        sprite::Animation::new(shooter.shared.anim.clone()),
                                        shooter.shared.hitbox,
                                        shooter.proj.clone(),
                                        velocity,
                                        game::LayerDisp(0.3),
                                        SpriteBundle::default(),
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
                        pos.x += 0.1 * velocity.x;
                        pos.y += 0.1 * velocity.y;
                        pos.z += 0.1 * velocity.z;
                    }
                    audio.play(shooter.audio.random());
                }
            }
        })
}
