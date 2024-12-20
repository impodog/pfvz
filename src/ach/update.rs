use crate::prelude::*;

pub(super) struct AchUpdatePlugin;

impl Plugin for AchUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewAchievement>();
        app.add_event::<SpawnAchievement>();
        app.init_resource::<PlayAudioQueue>();
        app.add_systems(Update, (play_audio,));
        app.add_systems(PostUpdate, (update_achievement, spawn_achievement_info));
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct NewAchievement(pub ach::AchId);

#[derive(Debug, Clone, Copy)]
pub enum SpawnAchievementOption {
    Banner,
    Page(usize),
}
#[derive(Event, Debug, Clone, Copy)]
pub struct SpawnAchievement {
    /// The anchored corner of the achievement
    pub pos: Vec3,
    pub ach: ach::AchId,
    pub option: SpawnAchievementOption,
    pub anchor: Anchor,
}

#[derive(Component)]
pub struct AchievementMarker;

fn hide_string(pat: &str) -> String {
    let mut s = String::new();
    s.reserve(pat.len());
    pat.chars().for_each(|_| {
        s.push('?');
    });
    s
}

fn spawn_achievement_info(
    mut commands: Commands,
    mut e_ach: EventReader<SpawnAchievement>,
    save: Res<save::Save>,
    font: Res<assets::DefaultFont>,
    italic: Res<assets::ItalicsFont>,
    achievements: Res<ach::Achievements>,
) {
    e_ach.read().for_each(|spawn| {
        if let Some(ach) = achievements.get(&spawn.ach) {
            // Test if the achievement is unlocked, or if the achievement has special represent
            // visibility
            let (name, name_color, desc, desc_color) = if save.ach.contains(&spawn.ach) {
                (
                    format!("{}\n", ach.name),
                    Color::LinearRgba(LinearRgba::new(0.05, 1.0, 0.2, 1.0)),
                    ach.desc.clone(),
                    Color::WHITE,
                )
            } else {
                let (name, desc) = match ach.vis {
                    ach::AchVisibility::Full => (format!("{}\n", ach.name), ach.desc.clone()),
                    ach::AchVisibility::NameOnly => {
                        (format!("{}\n", ach.name), hide_string(&ach.desc))
                    }
                    ach::AchVisibility::Hidden => {
                        let mut name = hide_string(&ach.name);
                        name.push('\n');
                        (name, hide_string(&ach.desc))
                    }
                };
                (
                    name,
                    Color::LinearRgba(LinearRgba::new(0.9, 0.9, 0.9, 1.0)),
                    desc,
                    Color::LinearRgba(LinearRgba::new(0.95, 0.95, 0.95, 1.0)),
                )
            };

            let wrap = desc.starts_with('?');
            let mut text = Text::from_sections([
                TextSection::new(
                    name,
                    TextStyle {
                        font: font.0.clone(),
                        font_size: ACH_SIZE.y / 7.0,
                        color: name_color,
                    },
                ),
                TextSection::new(
                    desc,
                    TextStyle {
                        font: italic.0.clone(),
                        font_size: ACH_SIZE.y / 8.0,
                        color: desc_color,
                    },
                ),
            ]);
            if wrap {
                text.linebreak_behavior = bevy::text::BreakLineOn::AnyCharacter;
            }

            let mut commands = commands.spawn((
                Text2dBundle {
                    text,
                    transform: Transform::from_translation(spawn.pos),
                    text_anchor: spawn.anchor,
                    text_2d_bounds: bevy::text::Text2dBounds {
                        size: ACH_SIZE.with_y(f32::INFINITY),
                    },
                    ..Default::default()
                },
                AchievementMarker,
            ));
            match spawn.option {
                SpawnAchievementOption::Banner => {
                    commands.insert(level::Banner::new(Duration::from_secs(5)));
                }
                SpawnAchievementOption::Page(page) => {
                    commands.insert(ach::show::AchPageIndex(page));
                    if page != 0 {
                        commands.insert(Visibility::Hidden);
                    }
                }
            }
        }
    });
}

#[derive(Resource, Default)]
struct PlayAudioQueue(bool);

fn play_audio(
    mut audio_queue: ResMut<PlayAudioQueue>,
    audio: Res<Audio>,
    audio_items: Res<assets::AudioItems>,
) {
    if audio_queue.0 {
        audio_queue.0 = false;
        audio.play(audio_items.ach.random());
    }
}

fn update_achievement(
    mut e_ach: EventReader<NewAchievement>,
    mut save: ResMut<save::Save>,
    mut e_spawn: EventWriter<SpawnAchievement>,
    mut audio_queue: ResMut<PlayAudioQueue>,
    q_ach: Query<(), With<AchievementMarker>>,
) {
    let mut event_count = 0;
    e_ach.read().for_each(|ach| {
        // Only spawn banner when the achievement is recently unlocked
        if save.ach.insert(ach.0) {
            let count = event_count + q_ach.iter().count();
            e_spawn.send(SpawnAchievement {
                pos: Vec3::new(
                    -LOGICAL_WIDTH / 2.0,
                    -LOGICAL_HEIGHT / 2.0 + ACH_SIZE.y * count as f32,
                    14.37 + 3.0,
                ),
                ach: ach.0,
                option: SpawnAchievementOption::Banner,
                anchor: Anchor::BottomLeft,
            });
            audio_queue.0 = true;
            event_count += 1;
        }
    });
}
