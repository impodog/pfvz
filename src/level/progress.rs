use crate::prelude::*;

pub(super) struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_progress_bar,));
        app.add_systems(
            PostUpdate,
            (modify_progress_bar,).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Resource, Debug, Clone, Copy)]
struct ProgressBarRect(game::Position, game::Position);

#[derive(Component)]
struct ProgressBarOverlayMarker;

fn init_progress_bar(
    mut commands: Commands,
    chunks: Res<assets::SpriteChunks>,
    level: Res<level::Level>,
    level_index: Res<level::LevelIndex>,
    font: Res<assets::DefaultFont>,
    display: Res<game::Display>,
) {
    let mut corner = level.config.layout.half_size_vec2() + Vec2::new(0.2, 0.2);
    corner.y = -corner.y;
    commands.insert_resource(ProgressBarRect((&corner).into(), (&PROGRESS_SIZE).into()));
    commands.spawn((
        game::Position::from(&corner),
        game::HitBox::from(&PROGRESS_SIZE),
        SpriteBundle {
            texture: chunks.white.clone(),
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                color: Color::LinearRgba(LinearRgba::new(0.8, 0.1, 0.1, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        ProgressBarOverlayMarker,
        game::Position::from(&corner),
        game::HitBox::from(&PROGRESS_SIZE).with_width(0.0),
        SpriteBundle {
            texture: chunks.white.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                color: Color::LinearRgba(LinearRgba::new(0.0, 1.0, 0.2, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        game::Position::from(&corner).move_by(-0.5, 0.0),
        Text2dBundle {
            text: Text::from_section(
                format!("{}", level_index.as_ref()),
                TextStyle {
                    font: font.0.clone(),
                    font_size: display.ratio * PROGRESS_SIZE.y,
                    color: Color::LinearRgba(LinearRgba::WHITE),
                },
            ),
            text_anchor: Anchor::TopRight,
            ..Default::default()
        },
    ));
}

fn modify_progress_bar(
    rect: Res<ProgressBarRect>,
    mut q_overlay: Query<&mut game::HitBox, With<ProgressBarOverlayMarker>>,
    mut next_wave: EventReader<level::RoomNextWave>,
    level: Res<level::Level>,
) {
    next_wave.read().for_each(|wave| {
        let wave = wave.0;
        q_overlay.iter_mut().for_each(|mut hitbox| {
            hitbox.width = (rect.1.x * (wave + 1) as f32) / level.waves.len() as f32;
        });
    });
}
