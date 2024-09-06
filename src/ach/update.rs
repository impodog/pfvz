use crate::prelude::*;

pub(super) struct AchUpdatePlugin;

impl Plugin for AchUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewAchievement>();
        app.add_event::<SpawnAchievement>();
        app.add_systems(Update, (update_achievement,));
        app.add_systems(PostUpdate, (spawn_achievement_info,));
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct NewAchievement(pub ach::AchId);

#[derive(Event, Debug, Clone, Copy)]
pub struct SpawnAchievement {
    /// The bottom left corner of the achievement
    pos: Vec3,
    ach: ach::AchId,
    banner: bool,
}

#[derive(Component)]
pub struct AchievementMarker;

fn hide_string(pat: &str) -> String {
    let mut s = String::new();
    s.reserve(pat.len());
    (0..s.len()).for_each(|_| {
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
                    Color::LinearRgba(LinearRgba::new(0.0, 0.9, 0.9, 1.0)),
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
            let mut commands = commands.spawn((
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            name,
                            TextStyle {
                                font: font.0.clone(),
                                font_size: ACH_SIZE.y / 10.0,
                                color: name_color,
                            },
                        ),
                        TextSection::new(
                            desc,
                            TextStyle {
                                font: italic.0.clone(),
                                font_size: ACH_SIZE.y / 12.0,
                                color: desc_color,
                            },
                        ),
                    ]),
                    transform: Transform::from_translation(spawn.pos),
                    text_anchor: Anchor::BottomLeft,
                    text_2d_bounds: bevy::text::Text2dBounds {
                        size: ACH_SIZE.with_y(f32::INFINITY),
                    },
                    ..Default::default()
                },
                AchievementMarker,
            ));
            if spawn.banner {
                commands.insert(level::Banner::new(Duration::from_secs(5)));
            }
        }
    });
}

fn update_achievement(
    mut e_ach: EventReader<NewAchievement>,
    mut save: ResMut<save::Save>,
    mut e_spawn: EventWriter<SpawnAchievement>,
    audio: Res<Audio>,
    audio_items: Res<assets::AudioItems>,
    q_ach: Query<(), With<AchievementMarker>>,
) {
    e_ach.read().for_each(|ach| {
        // Only spawn banner when the achievement is recently unlocked
        if save.ach.insert(ach.0) {
            let count = q_ach.iter().count();
            e_spawn.send(SpawnAchievement {
                pos: Vec3::new(
                    -LOGICAL_WIDTH / 2.0,
                    -LOGICAL_HEIGHT / 2.0 + ACH_SIZE.y * count as f32,
                    14.37 + 3.0,
                ),
                ach: ach.0,
                banner: true,
            });
            audio.play(audio_items.ach.random());
        }
    });
}
