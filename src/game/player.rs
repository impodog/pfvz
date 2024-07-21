use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::GlobalStates::Play),
            (init_player_status, show_selection, init_sun),
        );
        app.add_systems(PostStartup, (init_highlighter,));
        app.add_systems(
            Update,
            (update_highlight, update_select, update_sun)
                .run_if(in_state(info::GlobalStates::Play)),
        );
        app.init_resource::<Sun>();
        app.init_resource::<Selection>();
        app.init_resource::<Selecting>();
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct Sun(pub u32);

#[derive(Resource, Serialize, Deserialize, Default, Debug, Clone, Deref, DerefMut)]
pub struct Selection(pub Vec<Id>);

#[derive(Resource, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Selecting(pub usize);

impl Default for Selecting {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

#[derive(Component)]
struct SelectionMarker;

#[derive(Component)]
struct SelectionHighlighter;

#[derive(Component)]
struct SunIndicator;

fn init_highlighter(mut commands: Commands, chunks: Res<assets::SpriteChunks>) {
    commands.spawn((
        SelectionHighlighter,
        game::Position::default(),
        SpriteBundle {
            texture: chunks.highlight.clone(),
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.0, 0.0, 14.37 + 1.0),
            sprite: Sprite {
                color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 0.5)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn init_player_status(mut commands: Commands, level: Res<level::Level>) {
    commands.insert_resource(Sun(level.config.sun));
    commands.insert_resource(Selection::default());
    commands.insert_resource(Selecting::default());
}

fn show_selection(
    mut commands: Commands,
    sel: Res<Selection>,
    map: Res<game::CreatureMap>,
    display: Res<game::Display>,
    q_sel: Query<Entity, With<SelectionMarker>>,
) {
    if sel.is_changed() {
        q_sel.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
        for (i, id) in sel.iter().enumerate() {
            if let Some(creature) = map.get(id) {
                commands.spawn((
                    SelectionMarker,
                    SpriteBundle {
                        texture: creature
                            .anim
                            .frames
                            .first()
                            .expect("Empty animation!")
                            .clone(),
                        sprite: Sprite {
                            custom_size: Some(SLOT_SIZE * display.ratio),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    sprite::SlotIndex(i).into_position(display.ratio),
                ));
            } else {
                warn!("Selected non-existing id: {}", id);
            }
        }
    }
}

fn update_highlight(
    selecting: Res<Selecting>,
    display: Res<game::Display>,
    mut q_highlight: Query<(&mut game::Position, &mut Visibility), With<SelectionHighlighter>>,
) {
    if selecting.is_changed() {
        let (mut pos, mut visibility) = q_highlight.single_mut();
        if selecting.0 == usize::MAX {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
            *pos = sprite::SlotIndex(selecting.0).into_position(display.ratio);
        }
    }
}

fn update_select(
    mut selecting: ResMut<Selecting>,
    cursor: Res<info::CursorInfo>,
    display: Res<game::Display>,
    save: Res<save::Save>,
) {
    if cursor.left {
        if let Some(index) = sprite::SlotIndex::from_position(cursor.pos, display.ratio) {
            if index.0 < save.slots.0 {
                selecting.0 = index.0;
                info!("Selecting {}", selecting.0);
            } else {
                selecting.0 = usize::MAX;
            }
        } else {
            selecting.0 = usize::MAX;
        }
    } else if cursor.right {
        selecting.0 = usize::MAX;
    }
}

fn init_sun(mut commands: Commands, display: Res<game::Display>, font: Res<assets::DefaultFont>) {
    let mut pos = sprite::SlotIndex(0).into_position(display.ratio);
    pos.x -= SLOT_SIZE.x;
    commands.spawn((
        SunIndicator,
        pos,
        Text2dBundle {
            text: Text::from_section(
                "99999",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 40.0,
                    color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 1.0)),
                },
            ),
            ..Default::default()
        },
    ));
}

fn update_sun(mut sun: ResMut<Sun>, mut q_sun: Query<&mut Text, With<SunIndicator>>) {
    if sun.is_changed() {
        if sun.0 > 99999 {
            sun.0 = 99999;
        }
        if let Ok(mut text) = q_sun.get_single_mut() {
            text.sections[0].value = format!("{}", sun.0);
        }
    }
}
