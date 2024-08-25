use crate::prelude::*;

pub(super) struct MenuCreditsPlugin;

impl Plugin for MenuCreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::MenuStates::Credits), (spawn_credits,));
        app.add_systems(
            Update,
            (scroll_credits, exit_credits_by_esc).run_if(when_state!(credits)),
        );
        app.add_systems(OnExit(info::MenuStates::Credits), (despawn_credits,));
    }
}

#[derive(Component)]
pub struct CreditsMarker;

fn spawn_credits(mut commands: Commands, font: Res<assets::ChunksFont>) {
    commands.spawn((
        CreditsMarker,
        Text2dBundle {
            text: Text::from_section(
                std::fs::read_to_string("assets/text/credits.txt")
                    .expect("cannot read utf-8 assets/text/credits.txt"),
                TextStyle {
                    font: font.0.clone(),
                    font_size: 50.0,
                    color: Color::LinearRgba(LinearRgba::WHITE),
                },
            ),
            text_2d_bounds: bevy::text::Text2dBounds {
                size: LOGICAL.with_y(f32::INFINITY),
            },
            text_anchor: Anchor::TopCenter,
            transform: Transform::from_xyz(0.0, -LOGICAL_HEIGHT / 2.0, 14.37 + 2.0),
            ..Default::default()
        },
    ));
}

fn scroll_credits(
    mut q_credits: Query<&mut Transform, With<CreditsMarker>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<config::FrameTime>,
) {
    if let Ok(mut transform) = q_credits.get_single_mut() {
        let number = key.get_pressed().size_hint().0 + 1;
        transform.translation.y += time.diff() * LOGICAL_HEIGHT / 15.0 * number as f32;
    }
}

fn exit_credits_by_esc(
    mut menu: ResMut<NextState<info::MenuStates>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        menu.set(info::MenuStates::Main);
    }
}

fn despawn_credits(mut commands: Commands, q_credits: Query<Entity, With<CreditsMarker>>) {
    for entity in q_credits.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
