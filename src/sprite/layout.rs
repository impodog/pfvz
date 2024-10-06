use crate::prelude::*;

pub(super) struct SpriteLayoutPlugin;

impl Plugin for SpriteLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (spawn_layout,));
    }
}

#[derive(Debug, Clone, Copy, Deref, DerefMut)]
pub struct SlotIndex(pub usize);
impl SlotIndex {
    pub fn from_position(pos: game::Position, ratio: f32) -> Option<Self> {
        let x = (pos.x + LOGICAL_WIDTH / ratio / 2.0 - 1.0) / SLOT_SIZE.x;
        let x = (x + 0.5) as usize;
        let y_mid = LOGICAL_HEIGHT / ratio / 2.0 - SLOT_SIZE.y;
        if pos.y >= y_mid - SLOT_SIZE.y / 2.0 && pos.y <= y_mid + SLOT_SIZE.y / 2.0 {
            Some(SlotIndex(x))
        } else {
            None
        }
    }

    pub fn into_position(self, ratio: f32) -> game::Position {
        game::Position {
            x: -LOGICAL_WIDTH / ratio / 2.0 + (self.0 as f32) * SLOT_SIZE.x + 1.0,
            y: LOGICAL_HEIGHT / ratio / 2.0 - SLOT_SIZE.y,
            ..Default::default()
        }
    }
}

fn spawn_layout(
    mut commands: Commands,
    sp_chunks: Res<assets::SpriteChunks>,
    sp_layout: Res<assets::SpriteLayouts>,
    display: Res<game::Display>,
    level: Res<level::Level>,
    slots: Res<level::LevelSlots>,
) {
    let layout = level.config.layout;
    let size = layout.size();
    let half_x = size.0 as f32 / 2.0 - 0.5;
    let half_y = size.1 as f32 / 2.0 - 0.5;
    for row in 0..size.0 {
        for lane in 0..size.1 {
            let pos = game::Position {
                x: (row as f32 - half_x),
                y: (lane as f32 - half_y),
                z: level.config.layout.get_disp(row),
                ..Default::default()
            };
            let index = level.config.layout.get_layout(row, lane);
            commands.spawn((
                SpriteBundle {
                    texture: sp_layout
                        .get(&layout)
                        .get(index)
                        .cloned()
                        .unwrap_or_default(),
                    transform: Transform::from_xyz(0.0, 0.0, -14.37),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(display.ratio, display.ratio)),
                        color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 0.5)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                pos,
            ));
        }
    }
    let is_conveyor = level.conveyor.is_some();
    for index in 0..slots.0 {
        let pos = SlotIndex(index).into_position(display.ratio);
        let texture = if is_conveyor {
            if index == 0 {
                sp_chunks.conveyor_left.clone()
            } else if index == slots.0 - 1 {
                sp_chunks.conveyor_right.clone()
            } else {
                sp_chunks.conveyor_mid.clone()
            }
        } else {
            sp_chunks.slot.clone()
        };
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 14.37),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(SLOT_SIZE * display.ratio),
                    ..Default::default()
                },
                ..Default::default()
            },
            pos,
        ));
    }
}
