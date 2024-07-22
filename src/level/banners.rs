use crate::prelude::*;

pub(super) struct LevelBannersPlugin;

impl Plugin for LevelBannersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (banner_work,));
        app.add_systems(
            Update,
            (spawn_wave_banners,).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Banner(Timer);
impl Banner {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

fn banner_work(
    mut commands: Commands,
    time: Res<config::FrameTime>,
    mut q_banner: Query<(Entity, &mut Banner)>,
) {
    q_banner.iter_mut().for_each(|(entity, mut banner)| {
        banner.tick(time.delta());
        if banner.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    });
}

fn spawn_wave_banners(
    mut commands: Commands,
    chunks: Res<assets::SpriteChunks>,
    mut next_wave: EventReader<level::RoomNextWave>,
    level: Res<level::Level>,
) {
    next_wave.read().for_each(|wave| {
        let wave = wave.0;
        if wave == level.waves.len() - 1 {
            commands.spawn((
                game::Position::new_xy(0.0, 0.0),
                Banner::new(Duration::from_millis(4000)),
                SpriteBundle {
                    texture: chunks.final_wave.clone(),
                    ..Default::default()
                },
            ));
        } else if level.waves[wave].big {
            commands.spawn((
                game::Position::new_xy(0.0, 0.0),
                Banner::new(Duration::from_millis(4000)),
                SpriteBundle {
                    texture: chunks.alert.clone(),
                    ..Default::default()
                },
            ));
        }
    });
}
