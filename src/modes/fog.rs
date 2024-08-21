use crate::prelude::*;

pub(super) struct ModesFogPlugin;

impl Plugin for ModesFogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecalculateFog>();
        app.add_event::<RecalculateFogByTimer>();
        app.init_resource::<RemoveFogTimer>();
        app.add_systems(OnEnter(info::PlayStates::Gaming), (spawn_fog,));
        app.add_systems(
            Update,
            (test_fog_change, remove_fog, timer_tick).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct RemoveFog(pub game::PositionRange);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct RemoveFogTimer(pub Timer);
impl Default for RemoveFogTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(0.0), TimerMode::Once))
    }
}

#[derive(Component)]
pub struct FogMarker;

#[derive(Event)]
pub struct RecalculateFog;

/// This hides all fog if the blover timer is not complete
/// When you add time to the timer, please send this event
#[derive(Event)]
pub struct RecalculateFogByTimer;

fn spawn_fog(
    mut commands: Commands,
    level: Res<level::Level>,
    factors: Res<collectible::ItemFactors>,
    items: Res<assets::SpriteItems>,
) {
    if level.config.layout.has_fog() {
        let size = level.config.layout.size();
        for row in factors.fog.start..=size.0 {
            for col in 0..size.1 {
                let pos = level.config.layout.coordinates_to_position(row, col);
                commands.spawn((
                    FogMarker,
                    pos,
                    game::HitBox::new(1.0, 1.0),
                    sprite::Animation::new(items.fog.clone()),
                    SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 14.37 - 1.0),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

#[derive(Default, Deref, DerefMut)]
struct FogRelevant(BTreeSet<Entity>);

fn test_fog_change(
    q_fog: Query<Entity, With<RemoveFog>>,
    mut relevant: Local<FogRelevant>,
    mut e_fog: EventWriter<RecalculateFog>,
) {
    let changed = q_fog.iter().any(|entity| !relevant.contains(&entity))
        || relevant.iter().any(|entity| q_fog.get(*entity).is_err());
    if changed {
        **relevant = q_fog.iter().collect();
        e_fog.send(RecalculateFog);
    }
}

fn timer_tick(
    mut timer: ResMut<RemoveFogTimer>,
    time: Res<config::FrameTime>,
    mut e_fog_timer: EventReader<RecalculateFogByTimer>,
    mut q_fog: Query<&mut Visibility, With<FogMarker>>,
    mut e_fog: EventWriter<RecalculateFog>,
) {
    timer.tick(time.delta());
    if e_fog_timer.read().next().is_some() {
        q_fog.par_iter_mut().for_each(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }
    if timer.just_finished() {
        e_fog.send(RecalculateFog);
    }
}

fn remove_fog(
    q_remove: Query<(&game::Position, &RemoveFog)>,
    q_fog: Query<Entity, With<FogMarker>>,
    q_pos: Query<(&game::Position, &game::HitBox)>,
    mut q_vis: Query<&mut Visibility>,
    mut e_fog: EventReader<RecalculateFog>,
    timer: Res<RemoveFogTimer>,
) {
    if !timer.finished() || e_fog.read().next().is_none() {
        return;
    }

    let fog = q_fog.iter().collect::<Vec<_>>();
    let mut vis = vec![true; fog.len()];
    q_remove.iter().for_each(|(pos, remove)| {
        let range = remove.0.clone() + *pos;
        for (entity, vis) in fog.iter().zip(vis.iter_mut()) {
            if let Ok((pos, hitbox)) = q_pos.get(*entity) {
                if range.contains(pos, hitbox) {
                    *vis = false;
                }
            }
        }
    });
    for (entity, vis) in fog.into_iter().zip(vis.into_iter()) {
        if let Ok(mut visibility) = q_vis.get_mut(entity) {
            *visibility = if vis {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}
