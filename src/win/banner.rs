use crate::prelude::*;

pub(super) struct WinBannerPlugin;

impl Plugin for WinBannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::GlobalStates::Win),
            (spawn_banner, get_new_plant, note_from_zombies),
        );
    }
}

fn spawn_banner(mut commands: Commands, chunks: Res<assets::SpriteChunks>) {
    commands.spawn((
        level::Banner::new(Duration::from_millis(3000)),
        game::Position::new_xy(0.0, 3.0),
        SpriteBundle {
            texture: chunks.you_win.clone(),
            ..Default::default()
        },
    ));
}

fn note_from_zombies(
    mut commands: Commands,
    level_index: Res<level::LevelIndex>,
    italics: Res<assets::ItalicsFont>,
    interface: Res<assets::TextInterface>,
    chunks: Res<assets::SpriteChunks>,
) {
    if (1..=6).contains(&level_index.stage) && level_index.level == 9 {
        commands.spawn((
            game::Position::new_xy(0.0, 2.0),
            Text2dBundle {
                text: Text::from_sections([TextSection::new(
                    interface.win.note.clone() + "\n\n",
                    TextStyle {
                        font: italics.0.clone(),
                        font_size: 80.0,
                        color: Color::LinearRgba(LinearRgba::new(0.0, 1.0, 1.0, 1.0)),
                    },
                )]),
                ..Default::default()
            },
        ));
        let texture = match level_index.stage {
            1 => chunks.note1.clone(),
            2 => chunks.note2.clone(),
            _ => Default::default(),
        };
        commands.spawn((
            game::Position::new_xy(0.0, -0.5),
            game::HitBox::new(6.0, 4.2),
            SpriteBundle {
                texture,
                ..Default::default()
            },
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn get_new_plant(
    mut commands: Commands,
    level: Res<level::Level>,
    interface: Res<assets::TextInterface>,
    font: Res<assets::DefaultFont>,
    italics: Res<assets::ItalicsFont>,
    creatures: Res<assets::TextCreatures>,
    map: Res<game::CreatureMap>,
    mut save: ResMut<save::Save>,
) {
    if let Some(modify) = &level.config.modify {
        if modify.give != 0 && !save.plants.contains(&modify.give) {
            save.plants.insert(modify.give);
            if let Some(desc) = creatures.desc.get(id_name(modify.give)) {
                commands.spawn((
                    game::Position::new_xy(0.0, 0.0),
                    Text2dBundle {
                        text: Text::from_sections([
                            TextSection::new(
                                interface.win.get_plant.clone() + "\n\n",
                                TextStyle {
                                    font: italics.0.clone(),
                                    font_size: 80.0,
                                    color: Color::LinearRgba(LinearRgba::new(0.0, 1.0, 1.0, 1.0)),
                                },
                            ),
                            TextSection::new(
                                desc.name.clone() + "\n",
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                desc.short.clone(),
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ]),
                        ..Default::default()
                    },
                ));
                if let Some(creature) = map.get(&modify.give) {
                    commands.spawn((
                        game::Position::new_xy(0.0, -2.0),
                        creature.hitbox,
                        SpriteBundle {
                            texture: creature.image.clone(),
                            ..Default::default()
                        },
                    ));
                } else {
                    warn!("Got a plant without creature, this is not good");
                }
            } else {
                warn!("Got a plant {} without proper description", modify.give);
            }
        }
    }
}
