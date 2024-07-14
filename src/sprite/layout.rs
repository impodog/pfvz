use crate::prelude::*;

pub(super) struct SpriteLayoutPlugin;

impl Plugin for SpriteLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (spawn_layout,));
    }
}

fn spawn_layout(
    mut commands: Commands,
    sp_chunks: Res<assets::SpriteChunks>,
    sp_layout: Res<assets::SpriteLayouts>,
    display: Res<game::Display>,
    level: Res<level::Level>,
    save: Res<save::Save>,
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
                ..Default::default()
            };
            let index = lane * size.0 + row;
            let picture = index % 2;
            commands.spawn((
                SpriteBundle {
                    texture: sp_layout.get(&layout).grass[picture].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -1437.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(display.ratio, display.ratio)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                pos,
            ));
        }
    }
    for pos in 1..=save.slots.0 {
        let pos = game::Position {
            x: -LOGICAL_WIDTH / display.ratio / 2.0 + pos as f32 * SLOT_SIZE.x,
            y: LOGICAL_HEIGHT / display.ratio / 2.0 - SLOT_SIZE.y,
            ..Default::default()
        };
        commands.spawn((
            SpriteBundle {
                texture: sp_chunks.slot.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1437.0),
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
