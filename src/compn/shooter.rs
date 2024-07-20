use crate::prelude::*;

pub struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (add_shooter_impl, shooter_work).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

// Anything that uses this shoots projectile of their ally
#[derive(Debug, Clone)]
pub struct ShooterShared {
    pub interval: Duration,
    pub vel: game::Velocity,
    pub proj: game::Projectile,
    pub require_zombie: bool,
    pub shared: Arc<game::ProjectileShared>,
}
#[derive(Component, Debug, Clone, Deref)]
pub struct Shooter(pub Arc<ShooterShared>);

#[derive(Component, Debug, Clone)]
struct ShooterImpl {
    timer: Timer,
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
    mut commands: Commands,
    time: Res<Time>,
    mut q_shooter: Query<(Entity, &Shooter, &mut ShooterImpl, &game::Position)>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_zpos: Query<&game::Position, With<game::Zombie>>,
) {
    q_shooter
        .iter_mut()
        .for_each(|(entity, shooter, mut work, pos)| {
            work.timer.tick(time.delta());
            if work.timer.just_finished() {
                if shooter.require_zombie {
                    let mut ok = false;
                    for zpos in q_zpos.iter() {
                        if pos.same_y(zpos) && zpos.x >= pos.x {
                            ok = true;
                            break;
                        }
                    }
                    if !ok {
                        return;
                    }
                }
                let mut commands = commands.spawn((
                    *pos,
                    sprite::Animation::new(shooter.shared.anim.clone()),
                    shooter.shared.hitbox,
                    shooter.proj.clone(),
                    shooter.vel,
                    SpriteBundle::default(),
                ));
                // Determines whether the projectile is plant(default) or zombie
                if q_zombie.get(entity).is_ok() {
                    commands.insert(game::ZombieRelevant);
                } else {
                    commands.insert(game::PlantRelevant);
                }
            }
        })
}
