use crate::prelude::*;

pub(super) struct CompnExplodePlugin;

impl Plugin for CompnExplodePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplodeEvent>();
        app.add_systems(
            Update,
            (
                add_no_loss,
                explode_work,
                add_never_kill_for_bomb,
                cherry_bomb_timer_work,
                potato_mine_timer_work,
            )
                .run_if(when_state!(gaming)),
        );
    }
}

#[derive(Debug, Clone)]
pub struct ExplodeShared {
    pub anim: Arc<sprite::FrameArr>,
    // The time the animation should exist before despawning
    pub animation_time: Duration,
    pub hitbox: game::HitBox,
    pub damage: u32,
}

#[derive(Component, Debug, Clone, Deref)]
pub struct Explode(pub Arc<ExplodeShared>);

#[derive(Event, Debug, Clone)]
pub struct ExplodeEvent {
    pub entity: Entity,
}

fn add_no_loss(mut commands: Commands, q_explode: Query<Entity, Added<Explode>>) {
    q_explode.iter().for_each(|entity| {
        commands.entity(entity).insert(game::NoCollisionLoss);
    });
}

#[allow(clippy::too_many_arguments)]
fn explode_work(
    mut commands: Commands,
    mut explode_event: EventReader<ExplodeEvent>,
    q_explode: Query<(&game::Position, &Explode)>,
    mut action: EventWriter<game::CreatureAction>,
    collision: Res<game::Collision>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_plant: Query<(), With<game::Plant>>,
    config: Res<config::Config>,
) {
    explode_event.read().for_each(|event| {
        if let Ok((pos, explode)) = q_explode.get(event.entity) {
            commands.spawn((
                *pos,
                sprite::Animation::new(explode.anim.clone()),
                explode.hitbox,
                level::Banner::new(explode.animation_time),
                SpriteBundle::default(),
            ));
            if q_zombie.get(event.entity).is_ok() {
                if let Some(set) = collision.get(&event.entity) {
                    set.iter().for_each(|entity| {
                        if q_plant.get(*entity).is_ok() {
                            action.send(game::CreatureAction::Damage(
                                *entity,
                                multiply_uf!(explode.damage, config.gamerule.damage.0),
                            ));
                        }
                    });
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if let Some(set) = collision.get(&event.entity) {
                    set.iter().for_each(|entity| {
                        if q_zombie.get(*entity).is_ok() {
                            action.send(game::CreatureAction::Damage(
                                *entity,
                                multiply_uf!(explode.damage, config.gamerule.damage.0),
                            ));
                        }
                    });
                }
            }
            commands.entity(event.entity).despawn_recursive();
        } else {
            warn!("Unable to execute explode event on {:?}", event)
        }
    });
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct CherryBombTimer(pub Timer);

fn add_never_kill_for_bomb(
    commands: ParallelCommands,
    q_bomb: Query<Entity, Added<CherryBombTimer>>,
) {
    q_bomb.par_iter().for_each(|entity| {
        commands.command_scope(|mut commands| {
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.try_insert(compn::NeverKillWhenActive);
            }
        });
    });
}

fn cherry_bomb_timer_work(
    explode_event: EventWriter<ExplodeEvent>,
    mut q_timer: Query<(
        Entity,
        &game::Overlay,
        &mut CherryBombTimer,
        &mut game::HitBox,
        &mut game::LogicPosition,
        &Explode,
    )>,
) {
    let explode_event = Mutex::new(explode_event);
    q_timer.iter_mut().for_each(
        |(entity, overlay, mut timer, mut hitbox, mut pos, explode)| {
            timer.tick(overlay.delta());
            let rate = timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32();
            *hitbox = explode.hitbox * rate;
            pos.disp.z = -hitbox.height / 2.0 + 0.5;
            if timer.just_finished() {
                explode_event.lock().unwrap().send(ExplodeEvent { entity });
            }
        },
    );
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct PotatoMineTimer {
    #[deref]
    pub timer: Timer,
    pub prepared: Arc<sprite::FrameArr>,
}

fn potato_mine_timer_work(
    mut explode_event: EventWriter<ExplodeEvent>,
    mut q_timer: Query<(
        Entity,
        &game::Overlay,
        &mut PotatoMineTimer,
        &mut sprite::Animation,
    )>,
    q_plant: Query<(), With<game::Plant>>,
    q_zombie: Query<(), With<game::Zombie>>,
    collision: Res<game::Collision>,
) {
    q_timer
        .iter_mut()
        .for_each(|(entity, overlay, mut timer, mut anim)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                anim.replace(timer.prepared.clone());
            }
            if timer.finished() {
                if let Some(coll) = collision.get(&entity) {
                    let ok = if q_plant.get(entity).is_ok() {
                        coll.iter().any(|zombie| q_zombie.get(*zombie).is_ok())
                    } else {
                        coll.iter().any(|plant| q_plant.get(*plant).is_ok())
                    };
                    if ok {
                        explode_event.send(ExplodeEvent { entity });
                    }
                }
            }
        });
}
