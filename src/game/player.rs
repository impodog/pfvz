use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowSelectionEvent>();
        // This pre-shows the selected plants
        app.add_systems(OnEnter(info::PlayStates::Cys), (show_selection_on_startup,));
        app.add_systems(
            OnEnter(info::PlayStates::Gaming),
            (
                init_player_status,
                show_selection_on_startup,
                init_sun,
                init_highlighter,
                spawn_cooldown_rect,
                show_shovel,
            ),
        );
        app.add_systems(
            PreUpdate,
            (show_selection,).run_if(in_state(info::GlobalStates::Play)),
        );
        app.add_systems(
            Update,
            (update_cooldown, update_cooldown_rect, modify_cooldown).run_if(when_state!(gaming)),
        );
        app.add_systems(
            PostUpdate,
            (update_highlight, update_select, update_sun).run_if(when_state!(gaming)),
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

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct SelectionCooldown(pub Vec<Timer>);
impl SelectionCooldown {
    pub fn get(&mut self, index: usize) -> &mut Timer {
        if self.0.len() < index + 1 {
            self.0.resize_with(index + 1, Default::default);
        }
        self.0.get_mut(index).unwrap()
    }

    pub fn get_option(&self, index: usize) -> Option<&Timer> {
        self.0.get(index)
    }
}

#[derive(Component, Debug, Clone)]
pub struct SelectionCooldownIndex(usize);

impl Default for Selecting {
    fn default() -> Self {
        Self(usize::MAX)
    }
}

#[derive(Component)]
pub struct SelectionMarker;

#[derive(Component)]
pub struct SelectionHighlighter;

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

fn init_player_status(
    mut commands: Commands,
    level: Res<level::Level>,
    selection: Res<game::Selection>,
    map: Res<game::CreatureMap>,
) {
    commands.insert_resource(Sun(level.config.sun));
    // NOTE: The selection is initialized in level load. See it for details
    // commands.insert_resource(Selection::default());
    commands.insert_resource(Selecting::default());

    let cooldown = selection
        .iter()
        .map(|id| {
            let cooldown = map
                .get(id)
                .map(|creature| creature.cooldown)
                .unwrap_or_default();
            let half = cooldown * 2.0 / 3.0;
            let half = if half < 10.0 { 0.0 } else { half };
            let mut timer = Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once);
            timer.set_elapsed(Duration::from_secs_f32(cooldown - half));
            timer
        })
        .collect();
    commands.insert_resource(SelectionCooldown(cooldown));
}

/// This event is used to manually call `show_selection`, so that the selection refreshes
#[derive(Event, Debug, Clone)]
pub struct ShowSelectionEvent;

fn show_selection_on_startup(mut event: EventWriter<ShowSelectionEvent>) {
    event.send(ShowSelectionEvent);
}

#[allow(clippy::too_many_arguments)]
fn show_selection(
    mut commands: Commands,
    mut event: EventReader<ShowSelectionEvent>,
    sel: Res<Selection>,
    map: Res<game::CreatureMap>,
    font: Res<assets::DefaultFont>,
    display: Res<game::Display>,
    q_sel: Query<Entity, With<SelectionMarker>>,
    level: Res<level::Level>,
) {
    // Only spawn on incoming events, usually sent by `show_selection_on_startup`
    if event.read().next().is_none() {
        return;
    }

    q_sel.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
    for (i, id) in sel.iter().enumerate() {
        if let Some(creature) = map.get(id) {
            let parent = commands
                .spawn((
                    SelectionMarker,
                    SpriteBundle {
                        texture: creature.image.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 14.37 + 1.0),
                        sprite: Sprite {
                            custom_size: Some(SLOT_SIZE * display.ratio),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    sprite::SlotIndex(i).into_position(display.ratio),
                ))
                .id();
            if !level.hide_sun() {
                commands
                    .spawn((
                        game::Position::new(0.0, -0.25, 0.0, 0.0),
                        Text2dBundle {
                            text: Text::from_section(
                                format!("{}", creature.cost),
                                TextStyle {
                                    font: font.0.clone(),
                                    font_size: 30.0,
                                    color: Color::LinearRgba(LinearRgba::new(0.1, 1.0, 1.0, 0.8)),
                                },
                            ),
                            // z=1.0 makes sure that the cost is shown above the selection image
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..Default::default()
                        },
                    ))
                    .set_parent(parent);
            }
        } else if *id != 0 {
            warn!("Attempting to show non-existing id in slots bar: {}", id);
        }
    }
}

fn show_shovel(
    mut commands: Commands,
    display: Res<game::Display>,
    chunks: Res<assets::SpriteChunks>,
    slots: Res<level::LevelSlots>,
) {
    commands.spawn((
        SpriteBundle {
            texture: chunks.shovel.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 14.37 + 1.0),
            sprite: Sprite {
                custom_size: Some(SLOT_SIZE * display.ratio),
                ..Default::default()
            },
            ..Default::default()
        },
        sprite::SlotIndex(slots.0).into_position(display.ratio),
    ));
}

fn update_cooldown(mut cooldown: ResMut<SelectionCooldown>, time: Res<config::FrameTime>) {
    for timer in cooldown.0.iter_mut() {
        timer.tick(time.delta());
    }
}

fn update_cooldown_rect(
    cooldown: Res<SelectionCooldown>,
    mut q_cooldown: Query<(&SelectionCooldownIndex, &mut Sprite)>,
    display: Res<game::Display>,
) {
    q_cooldown.par_iter_mut().for_each(|(index, mut sprite)| {
        if let Some(timer) = cooldown.get_option(index.0) {
            if !timer.finished() {
                let mut size = SLOT_SIZE * display.ratio;
                size.y *= 1.0 - timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32();
                sprite.custom_size = Some(size);
            }
        }
    });
}

fn spawn_cooldown_rect(
    mut commands: Commands,
    chunks: Res<assets::SpriteChunks>,
    display: Res<game::Display>,
    slots: Res<level::LevelSlots>,
    selection: Res<Selection>,
    map: Res<game::CreatureMap>,
) {
    (0..slots.0).for_each(|index| {
        let mut pos = sprite::SlotIndex(index).into_position(display.ratio);
        pos.x -= SLOT_SIZE.x / 2.0;
        pos.y -= SLOT_SIZE.y / 2.0;
        if selection.get(index).is_some_and(|id| map.get(id).is_some()) {
            commands.spawn((
                pos,
                SelectionCooldownIndex(index),
                SpriteBundle {
                    texture: chunks.cooldown.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 14.37 + 10.0),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 0.9)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    });
}

fn modify_cooldown(
    mut planter: EventReader<plants::PlanterEvent>,
    mut cooldown: ResMut<SelectionCooldown>,
    map: Res<game::CreatureMap>,
) {
    planter.read().for_each(|planter| {
        if let Some(creature) = map.get(&planter.id) {
            *cooldown.get(planter.index) =
                Timer::new(Duration::from_secs_f32(creature.cooldown), TimerMode::Once);
        } else {
            info!("Unable to update cooldown for planter id {}", planter.id);
        }
    });
}

fn update_highlight(
    selecting: Res<Selecting>,
    display: Res<game::Display>,
    mut q_highlight: Query<(&mut game::Position, &mut Visibility), With<SelectionHighlighter>>,
) {
    if selecting.is_changed() {
        if let Ok((mut pos, mut visibility)) = q_highlight.get_single_mut() {
            if selecting.0 == usize::MAX {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
                *pos = sprite::SlotIndex(selecting.0).into_position(display.ratio);
            }
        }
    }
}

fn update_select(
    mut selecting: ResMut<Selecting>,
    cursor: Res<info::CursorInfo>,
    key: Res<ButtonInput<KeyCode>>,
    display: Res<game::Display>,
    slots: Res<level::LevelSlots>,
) {
    if cursor.left {
        if let Some(index) = sprite::SlotIndex::from_position(cursor.pos, display.ratio) {
            // +1 for shovel
            if index.0 < slots.0 + 1 {
                selecting.0 = index.0;
            }
        }
        // When clicked left key on non-slot positions, do nothing
        // It may seem weird, but this avoids setting it too early so that planter cannot read the
        // selected index
    } else if cursor.right {
        selecting.0 = usize::MAX;
    }
    key.get_just_pressed().for_each(|key| {
        let index = match key {
            KeyCode::Digit1 => 0,
            KeyCode::Digit2 => 1,
            KeyCode::Digit3 => 2,
            KeyCode::Digit4 => 3,
            KeyCode::Digit5 => 4,
            KeyCode::Digit6 => 5,
            KeyCode::Digit7 => 6,
            KeyCode::Digit8 => 7,
            KeyCode::Digit9 => 8,
            KeyCode::Digit0 => 9,
            KeyCode::KeyS => slots.0,
            _ => usize::MAX,
        };
        if index < slots.0 + 1 {
            selecting.0 = index;
        }
    });
}

fn init_sun(
    mut commands: Commands,
    display: Res<game::Display>,
    font: Res<assets::DefaultFont>,
    level: Res<level::Level>,
) {
    if !level.hide_sun() {
        let mut pos = sprite::SlotIndex(0).into_position(display.ratio);
        pos.x -= SLOT_SIZE.x;
        commands.spawn((
            SunIndicator,
            pos,
            Text2dBundle {
                text: Text::from_section(
                    "CYAN",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 40.0,
                        color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 1.0)),
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 14.37),
                ..Default::default()
            },
        ));
    }
}

fn update_sun(
    mut sun: ResMut<Sun>,
    mut q_sun: Query<&mut Text, With<SunIndicator>>,
    level: Res<level::Level>,
) {
    if level.hide_sun() {
        sun.0 = SUN_MAGIC;
        return;
    }
    if sun.is_changed() {
        let value = if sun.0 == SUN_MAGIC {
            "inf".to_owned()
        } else {
            if sun.0 > 99999 {
                sun.0 = 99999;
            }
            format!("{}", sun.0)
        };
        if let Ok(mut text) = q_sun.get_single_mut() {
            text.sections[0].value = value;
        }
    }
}
