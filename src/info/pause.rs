use crate::prelude::*;

pub(super) struct InfoPausePlugin;

impl Plugin for InfoPausePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseEvent>().add_event::<PauseWorkEvent>();
        app.add_systems(Last, (pause_time,));
        app.add_systems(
            Update,
            (pause_by_key, rotate_pause_indicator, spawn_pause_indicator)
                .run_if(when_state!(gaming)),
        );
        app.add_systems(OnExit(info::PlayStates::Gaming), (unpause_by_quit,));
    }
}

#[derive(Event)]
pub enum PauseEvent {
    Toggle,
    Pause,
    Unpause,
}

#[derive(Event)]
pub enum PauseWorkEvent {
    Pause,
    Unpause,
}

#[derive(Component)]
pub struct PauseIndicator;

fn pause_time(
    mut event: EventReader<PauseEvent>,
    mut work: EventWriter<PauseWorkEvent>,
    mut time: ResMut<Time<Virtual>>,
    mut frame_time: ResMut<config::FrameTime>,
    mut audio: ResMut<Assets<AudioInstance>>,
) {
    let mut any = false;
    let paused = event.read().fold(time.is_paused(), |paused, event| {
        any = true;
        match event {
            PauseEvent::Toggle => !paused,
            PauseEvent::Pause => true,
            PauseEvent::Unpause => false,
        }
    });
    if any && paused != time.is_paused() {
        if paused {
            time.pause();
            audio.iter_mut().for_each(|(_id, instance)| {
                instance.pause(AudioTween::default());
            });
            work.send(PauseWorkEvent::Pause);
        } else {
            time.unpause();
            audio.iter_mut().for_each(|(_id, instance)| {
                instance.resume(AudioTween::default());
            });
            work.send(PauseWorkEvent::Unpause);
        }
        frame_time.pause(paused);
    }
}

fn pause_by_key(key: Res<ButtonInput<KeyCode>>, mut event: EventWriter<PauseEvent>) {
    if key.any_just_pressed([KeyCode::Space, KeyCode::KeyP]) {
        event.send(PauseEvent::Toggle);
    }
}

fn unpause_by_quit(mut event: EventWriter<PauseEvent>) {
    event.send(PauseEvent::Unpause);
}

fn spawn_pause_indicator(
    mut commands: Commands,
    mut work: EventReader<PauseWorkEvent>,
    chunks: Res<assets::SpriteChunks>,
    q_indicator: Query<Entity, With<PauseIndicator>>,
) {
    if let Some(work) = work.read().last() {
        match work {
            PauseWorkEvent::Pause => {
                commands.spawn((
                    SpriteBundle {
                        texture: chunks.pdog.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 14.37 + 3.0),
                        sprite: Sprite {
                            custom_size: Some(LOGICAL * (3.0 / 4.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    PauseIndicator,
                ));
            }
            PauseWorkEvent::Unpause => {
                q_indicator.iter().for_each(|entity| {
                    if let Some(commands) = commands.get_entity(entity) {
                        commands.despawn_recursive();
                    }
                });
            }
        }
    }
}

fn rotate_pause_indicator(
    mut q_indicator: Query<&mut Transform, With<PauseIndicator>>,
    factors: Res<collectible::ItemFactors>,
    time: Res<Time<Real>>,
) {
    q_indicator.par_iter_mut().for_each(|mut transform| {
        transform.rotate_x(factors.pause.rotate_speed * time.delta_seconds() * 0.333);
        transform.rotate_y(factors.pause.rotate_speed * time.delta_seconds());
        transform.rotate_z(factors.pause.rotate_speed * time.delta_seconds() * 0.777);
    });
}
