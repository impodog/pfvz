use crate::prelude::*;

pub(super) struct LevelBgmPlugin;

impl Plugin for LevelBgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (start_playing,));
        app.add_systems(OnExit(info::PlayStates::Gaming), (close_all_music,));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct BgmHandle(pub Handle<AudioInstance>);

fn start_playing(
    mut commands: Commands,
    audio: Res<Audio>,
    bgm: Res<assets::AudioBgm>,
    level: Res<level::Level>,
) {
    let music = match level.config.bgm {
        Some(ref str) => bgm.get_name(str),
        None => bgm.get_layout(level.config.layout),
    };
    if let Some(music) = music {
        commands.insert_resource(BgmHandle(audio.play(music.clone()).handle()));
    }
}

fn close_all_music(mut audio_instances: ResMut<Assets<AudioInstance>>) {
    audio_instances.iter_mut().for_each(|(_id, instance)| {
        instance.stop(AudioTween::default());
    });
}
