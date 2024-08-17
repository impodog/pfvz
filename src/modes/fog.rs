use crate::prelude::*;

pub(super) struct ModesFogPlugin;

impl Plugin for ModesFogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (spawn_fog,));
        app.add_systems(Update, (remove_fog,).run_if(when_state!(gaming)));
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct RemoveFog(pub game::PositionRange);

#[derive(Component)]
pub struct FogMarker;

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

#[derive(Debug, Clone, Deref, DerefMut)]
struct RemoveFogTimer(pub Timer);

impl Default for RemoveFogTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

fn remove_fog(
    q_remove: Query<(&game::Position, &RemoveFog)>,
    q_fog: Query<Entity, With<FogMarker>>,
    q_pos: Query<(&game::Position, &game::HitBox)>,
    mut q_vis: Query<&mut Visibility>,
    mut timer: Local<RemoveFogTimer>,
    time: Res<config::FrameTime>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
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
