use crate::prelude::*;

pub struct CompnShooterPlugin;

impl Plugin for CompnShooterPlugin {
    fn build(&self, app: &mut App) {
        initialize(&shooter_sound);
        app.add_systems(PostStartup, (init_shooter,));
        app.add_systems(
            PreUpdate,
            (add_shooter_impl, shooter_work).run_if(when_state!(gaming)),
        );
    }
}

lazy_static! {
    static ref shooter_sound: RwLock<Option<assets::AudioList>> = RwLock::new(None);
}

fn init_shooter(audio_plants: Res<assets::AudioPlants>) {
    *shooter_sound.write().unwrap() = Some(audio_plants.shooter.clone());
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
    pub audio: assets::AudioList,
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
    audio: Res<Audio>,
) {
    q_shooter
        .par_iter_mut()
        .for_each(|(entity, overlay, shooter, mut work, pos, hitbox)| {
            work.timer.tick(overlay.delta());
            if work.timer.just_finished() {
                let mut pos = (*pos).move_z(hitbox.height * -0.05);
                let range = shooter.proj.range.clone() + pos;
                if shooter.require_zombie {
                    let mut ok = false;
                    for (zombie_pos, zombie_hitbox) in q_zpos.iter() {
                        if range.contains(zombie_pos, zombie_hitbox) {
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
                    pos.x += 0.1 * shooter.velocity.x;
                }
                audio.play(shooter.audio.random());
            }
        })
}
