use crate::prelude::*;

pub(super) struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (add_shooter_impl, shooter_work));
    }
}

// Anything that uses this shoots projectile of their ally
#[derive(Component, Debug, Clone)]
pub struct Shooter {
    interval: Duration,
    proj: game::Projectile,
    shared: Arc<game::ProjectileShared>,
}

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
) {
    q_shooter
        .iter_mut()
        .for_each(|(entity, shooter, mut work, pos)| {
            work.timer.tick(time.delta());
            if work.timer.just_finished() {
                let mut commands = commands.spawn((
                    *pos,
                    sprite::Animation::new(shooter.shared.anim.clone()),
                    shooter.shared.hitbox,
                    shooter.proj.clone(),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some((&shooter.shared.hitbox).into()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
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
