use crate::prelude::*;
use game::GameItem;

pub(super) struct SpriteLayoutPlugin;

impl Plugin for SpriteLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (spawn_layout,));
    }
}

fn spawn_layout(
    mut commands: Commands,
    layout_sp: Res<assets::SpriteLayouts>,
    display: Res<game::Display>,
    level: Res<level::Level>,
) {
    let layout = level.config.layout;
    let size = layout.size();
    let half_x = size.0 as f32 / 2.0;
    let half_y = size.1 as f32 / 2.0;
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
                GameItem,
                SpriteBundle {
                    texture: layout_sp.get(&layout).grass[picture].clone(),
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
}
