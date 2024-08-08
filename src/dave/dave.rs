use crate::prelude::*;

pub(super) struct DaveDavePlugin;

impl Plugin for DaveDavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_text,));
        app.add_systems(OnEnter(info::PlayStates::Dave), (spawn_dave,));
        app.add_systems(
            Update,
            (spawn_text, click, add_craziness).run_if(when_state!(dave)),
        );
        app.add_systems(OnExit(info::PlayStates::Dave), (quit_dave,));
    }
}

#[derive(Component)]
pub struct DaveMarker;

#[derive(Component)]
pub struct Craziness;

#[derive(Component)]
pub struct DaveTextMarker;

#[derive(Resource, Default, Debug, Clone)]
pub struct DaveSaying {
    pub text: Vec<String>,
    pub cursor: usize,
}

fn init_text(mut commands: Commands) {
    commands.insert_resource(DaveSaying::default());
}

fn spawn_dave(
    mut commands: Commands,
    display: Res<game::Display>,
    mut next: ResMut<NextState<info::PlayStates>>,
    index: Res<level::LevelIndex>,
    text: Res<assets::TextDave>,
    chunks: Res<assets::SpriteChunks>,
) {
    if let Some(text) = text.get(index.as_ref()) {
        commands.insert_resource(DaveSaying {
            text: (**text).clone(),
            ..Default::default()
        });
        let scr = LOGICAL / display.ratio;
        commands.spawn((
            DaveMarker,
            Craziness,
            game::Position::new_xy(scr.x / 6.0, 0.0),
            game::HitBox::new(scr.x / 3.0, scr.y),
            game::Velocity::default(),
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                texture: chunks.dave.clone(),
                ..Default::default()
            },
        ));
    } else {
        next.set(info::PlayStates::Cys);
    }
}

fn add_craziness(mut q_dave: Query<(&game::Position, &mut game::Velocity), With<Craziness>>) {
    q_dave.iter_mut().for_each(|(pos, mut velocity)| {
        velocity.x = -pos.x / 5.0;
        if rand::thread_rng().gen_bool(0.05) {
            velocity.r = rand::thread_rng().gen_range(-0.3..0.3);
            velocity.x += rand::thread_rng().gen_range(-1.0..1.0);
        }
    });
}

fn spawn_text(
    mut commands: Commands,
    saying: Res<DaveSaying>,
    font: Res<assets::ItalicsFont>,
    q_text: Query<Entity, With<DaveTextMarker>>,
) {
    if saying.is_changed() {
        q_text.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
        if let Some(text) = saying.text.get(saying.cursor) {
            commands.spawn((
                DaveMarker,
                DaveTextMarker,
                Text2dBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 100.0,
                            color: Color::LinearRgba(LinearRgba::new(0.3, 0.6, 0.6, 1.0)),
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 4.0),
                    text_2d_bounds: bevy::text::Text2dBounds { size: LOGICAL },
                    ..Default::default()
                },
            ));
        }
    }
}

fn click(
    mut saying: ResMut<DaveSaying>,
    cursor: Res<info::CursorInfo>,
    mut next: ResMut<NextState<info::PlayStates>>,
) {
    if cursor.left {
        saying.cursor += 1;
        if saying.cursor >= saying.text.len() {
            next.set(info::PlayStates::Cys);
        }
    }
}

fn quit_dave(mut commands: Commands, q_dave: Query<Entity, With<DaveMarker>>) {
    q_dave.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
