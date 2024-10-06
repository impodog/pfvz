use crate::prelude::*;

pub(super) struct ChooseInfoPlugin;

impl Plugin for ChooseInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Cys), (init_info,));
        app.add_systems(OnExit(info::PlayStates::Cys), (despawn_info,));
    }
}

#[derive(Component)]
pub struct ChooseInfoMarker;

fn init_info(
    mut commands: Commands,
    level: Res<level::Level>,
    difficulty: Res<level::RoomDifficulty>,
    font: Res<assets::DefaultFont>,
    display: Res<game::Display>,
) {
    let corner = level
        .config
        .layout
        .coordinates_to_position(0, 0)
        .move_by(-1.0, 0.0);

    // Difficulty info, only shown in adventure levels
    if level.config.game.contains(&level::GameKind::Adventure) {
        let font_size = display.ratio / 3.0;
        let mut text = vec![TextSection::new(
            "Difficulty: ",
            TextStyle {
                font: font.0.clone(),
                font_size,
                color: Color::WHITE,
            },
        )];
        let factor = (difficulty.factor / 2.0).max(0.0);
        let x_bound = display.ratio * corner.x + LOGICAL_WIDTH / 2.0;
        let str = format!("{:.1}%", difficulty.factor * 100.0);
        if factor <= 1.0 {
            let color = LinearRgba::new(0.0, 1.0, 1.0, 1.0).mix(&LinearRgba::RED, factor);
            text.push(TextSection::new(
                str,
                TextStyle {
                    font: font.0.clone(),
                    font_size,
                    color: Color::LinearRgba(color),
                },
            ));
        } else {
            let acc = (factor - 1.0) / (str.len() as f32 - 1.0);
            let mut current_factor = 0.0;
            for ch in str.chars() {
                let color = LinearRgba::RED.mix(&LinearRgba::BLUE, current_factor);
                text.push(TextSection::new(
                    ch.to_string(),
                    TextStyle {
                        font: font.0.clone(),
                        font_size,
                        color: Color::LinearRgba(color),
                    },
                ));
                current_factor += acc;
            }
        }
        commands.spawn((
            ChooseInfoMarker,
            corner,
            Text2dBundle {
                text: Text::from_sections(text),
                text_anchor: Anchor::CenterRight,
                text_2d_bounds: bevy::text::Text2dBounds {
                    size: Vec2::new(x_bound, f32::INFINITY),
                },
                transform: Transform::from_xyz(0.0, 0.0, 14.37 + 3.0),
                ..Default::default()
            },
        ));
    }
}

fn despawn_info(mut commands: Commands, q_info: Query<Entity, With<ChooseInfoMarker>>) {
    q_info.iter().for_each(|entity| {
        if let Some(commands) = commands.get_entity(entity) {
            commands.despawn_recursive();
        }
    });
}
