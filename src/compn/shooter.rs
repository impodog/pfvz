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
/// The higher the return value of the predicate is, the more priority this target gets
#[derive(Clone)]
pub struct ShooterPredicate(
    pub  Arc<
        dyn Fn(
                (&game::Position, &game::Creature),
                (&game::Position, &game::Creature),
            ) -> std::cmp::Ordering
            + Send
            + Sync,
    >,
);
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
#[derive(Clone)]
pub struct ShooterShared {
    pub interval: Duration,
    pub velocity: ShooterVelocity,
    pub proj: game::Projectile,
    pub start: Vec<(game::Position, f32)>,
    pub times: usize,
    pub require_zombie: RequireZombie,
    /// Setting this to None(default) uses the default comparision
    /// Or, the one that returns the highest ordering is returned
    pub predicate: Option<ShooterPredicate>,
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
            predicate: None,
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
#[derive(Component, Clone, Deref)]
pub struct Shooter(pub Arc<ShooterShared>);
impl Shooter {
    pub fn replace(&mut self, shared: Arc<ShooterShared>) {
        self.0 = shared;
    }
}

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

/// Calculates the initial velocity needed to hit an object at distance diff in 3d math
/// with gravity, the velocity of the moving target, and the x orthogonal velocity of the initial specified
#[derive(Debug)]
pub struct TargetCalculator {
    pub diff: game::Position,
    pub target_vel: game::Velocity,
    pub x: f32,
    pub r: f32,
    pub g: f32,
}
impl TargetCalculator {
    pub fn calc(self) -> game::Velocity {
        let time = self.diff.x / (self.x - self.target_vel.x);
        let z = self.diff.z / time + 0.5 * self.g * time + self.target_vel.z;
        let y = self.diff.y / time + self.target_vel.y;
        game::Velocity::new(self.x, y, z, self.r)
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
    q_zpos: Query<(&game::Position, &game::HitBox), With<game::Zombie>>,
    q_zombie_creature: Query<
        (
            &game::Position,
            &game::HitBox,
            Option<&game::Velocity>,
            &game::Creature,
        ),
        With<game::Zombie>,
    >,
    q_plant_creature: Query<
        (
            &game::Position,
            &game::HitBox,
            Option<&game::Velocity>,
            &game::Creature,
        ),
        With<game::Plant>,
    >,
    config: Res<config::Config>,
    audio: Res<Audio>,
) {
    q_shooter
        .par_iter_mut()
        .for_each(|(entity, overlay, shooter, mut work, pos, hitbox)| {
            work.timer.tick(overlay.delta());
            if work.timer.just_finished() {
                let mut pos = (*pos).move_z(hitbox.height * -0.05);
                let is_zombie = q_zombie.get(entity).is_ok();

                // Test if the shooter should work
                let range = shooter.proj.range + pos;
                let ok = match shooter.require_zombie {
                    RequireZombie::No => true,
                    RequireZombie::InRange => q_zpos.iter().any(|(zombie_pos, zombie_hitbox)| {
                        range.contains(zombie_pos, zombie_hitbox)
                    }),
                    RequireZombie::RayCast => shooter.start.iter().any(|(start, angle)| {
                        use bevy::math::{bounding::Aabb2d, Ray2d};
                        let start = *start + pos;
                        let ray = Ray2d::new(
                            Vec2::new(start.x, start.y),
                            Vec2::new(angle.cos(), angle.sin()),
                        );
                        q_zpos.iter().any(|(zombie_pos, zombie_hitbox)| {
                            if shooter.proj.range.z.0 <= zombie_pos.z + zombie_hitbox.height / 2.0
                                && zombie_pos.z - zombie_hitbox.height / 2.0
                                    <= shooter.proj.range.z.1
                            {
                                let aabb = Aabb2d::new(
                                    Vec2::new(zombie_pos.x, zombie_pos.y),
                                    Vec2::new(zombie_hitbox.width, 0.8),
                                );
                                ray.intersects(&aabb)
                            } else {
                                false
                            }
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
                        // Selects a target entity to lob at
                        type CompareElem<'a, 'b> = &'a (
                            game::Position,
                            game::Velocity,
                            &'b game::HitBox,
                            &'b game::Creature,
                        );
                        let compare =
                            |(lhs_pos, _, _, lhs_creature): CompareElem,
                             (rhs_pos, _, _, rhs_creature): CompareElem| {
                                // Reversed comparing makes sure the first zombie is hit
                                if let Some(ref predicate) = shooter.predicate {
                                (*predicate.0)((lhs_pos, lhs_creature), (rhs_pos, rhs_creature))
                                } else {
                                    rhs_pos.x.partial_cmp(&lhs_pos.x).unwrap()
                                }
                            };
                        let target = if is_zombie {
                            q_plant_creature
                                .iter()
                                .filter_map(
                                    |(other_pos, other_hitbox, other_velocity, other_creature)| {
                                        if range.contains(other_pos, other_hitbox) {
                                            Some((
                                                *other_pos,
                                                other_velocity.copied().unwrap_or_default(),
                                                other_hitbox,
                                                other_creature,
                                            ))
                                        } else {
                                            None
                                        }
                                    },
                                )
                                .max_by(compare)
                        } else {
                            q_zombie_creature
                                .iter()
                                .filter_map(
                                    |(other_pos, other_hitbox, other_velocity, other_creature)| {
                                        if range.contains(other_pos, other_hitbox) {
                                            Some((
                                                *other_pos,
                                                other_velocity.copied().unwrap_or_default(),
                                                other_hitbox,
                                                other_creature,
                                            ))
                                        } else {
                                            None
                                        }
                                    },
                                )
                                .max_by(compare)
                        };
                        // Calculate the required velocity
                        target.map(|(target, velocity, _, _)| {
                            let diff = target - pos;
                            TargetCalculator {
                                diff,
                                target_vel: velocity,
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
                                        game::LayerDisp(0.03),
                                        SpriteBundle::default(),
                                    ));
                                    // Determines whether the projectile is plant(default) or zombie
                                    if is_zombie {
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
