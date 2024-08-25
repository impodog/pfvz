use crate::prelude::*;

pub(super) struct LevelBgmPlugin;

impl Plugin for LevelBgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::PlayStates::Gaming),
            (init_status, start_playing).chain(),
        );
        app.add_systems(OnExit(info::PlayStates::Gaming), (close_all_music,));
        app.add_systems(PostUpdate, (switch_exciting,).run_if(when_state!(gaming)));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct BgmHandle(pub Handle<AudioInstance>);

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub enum BgmStatus {
    #[default]
    Single,
    Normal,
    Exciting,
}

fn init_status(mut commands: Commands) {
    commands.remove_resource::<BgmHandle>();
    commands.insert_resource(BgmStatus::default());
}

fn start_playing(
    mut commands: Commands,
    audio: Res<Audio>,
    bgm: Res<assets::AudioBgm>,
    level: Res<level::Level>,
) {
    let music = match level.config.bgm {
        Some(ref str) => {
            commands.insert_resource(BgmStatus::Single);
            bgm.get_name(str)
        }
        None => {
            commands.insert_resource(BgmStatus::Normal);
            bgm.get_layout(level.config.layout).map(|bgm| &bgm.normal)
        }
    };
    if let Some(music) = music {
        commands.insert_resource(BgmHandle(audio.play(music.clone()).handle()));
    }
}

fn switch_exciting(
    mut status: ResMut<BgmStatus>,
    bgm: Res<assets::AudioBgm>,
    level: Res<level::Level>,
    audio: Res<Audio>,
    factors: Res<collectible::ItemFactors>,
    handle: Option<ResMut<BgmHandle>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_zombies: Query<(), (With<game::Zombie>, Without<game::NotInvasive>)>,
) {
    if *status == BgmStatus::Single {
        return;
    }
    if let Some(mut handle) = handle {
        let count = q_zombies.iter().count();
        let next_status = if count >= factors.exciting.zombies {
            BgmStatus::Exciting
        } else {
            BgmStatus::Normal
        };
        if *status != next_status {
            if let Some(layout) = bgm.get_layout(level.config.layout) {
                if let Some(instance) = audio_instances.get_mut(handle.id()) {
                    let mut time = instance.state().position().unwrap_or_default();
                    if next_status == BgmStatus::Normal {
                        time += layout.begin;
                    } else {
                        time -= layout.begin;
                        if time < 0.0 {
                            time = 0.0;
                        }
                    }

                    instance.stop(AudioTween::new(
                        Duration::from_secs_f32(0.5),
                        AudioEasing::OutPowi(1),
                    ));
                    let music = if next_status == BgmStatus::Normal {
                        layout.normal.clone()
                    } else {
                        layout.exciting.clone()
                    };
                    handle.0 = audio.play(music).start_from(time).handle();
                }
            }
            *status = next_status;
        }
    }
}

fn close_all_music(mut audio_instances: ResMut<Assets<AudioInstance>>) {
    audio_instances.iter_mut().for_each(|(_id, instance)| {
        instance.stop(AudioTween::default());
    });
}
