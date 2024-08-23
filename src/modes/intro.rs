use crate::prelude::*;

pub(super) struct ModesIntroPlugin;

impl Plugin for ModesIntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Cys), (switch_to_intro,));
        app.add_systems(OnEnter(info::PlayStates::Intro), (spawn_present_by,));
        app.add_systems(Update, (present_by_timer_tick,).run_if(when_state!(intro)));
        app.add_systems(OnExit(info::PlayStates::Intro), (despawn_present_by,));
        app.add_systems(OnExit(info::GlobalStates::Play), (despawn_present_by,));
        app.add_systems(OnEnter(info::PlayStates::Gaming), (spawn_title,));
        app.add_systems(
            Update,
            (remove_banners, title_timer_tick).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component)]
pub struct PresentBy;

#[derive(Resource, Deref, DerefMut)]
struct PresentByTimer(Timer);

impl Default for PresentByTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(4.0, TimerMode::Once))
    }
}

#[derive(Resource, Deref, DerefMut)]
struct TitleTimer(Timer);

impl Default for TitleTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

fn switch_to_intro(mut play: ResMut<NextState<info::PlayStates>>, level: Res<level::Level>) {
    if level.config.game.contains(&level::GameKind::Intro) {
        play.set(info::PlayStates::Intro);
    }
}

fn spawn_present_by(
    mut commands: Commands,
    font: Res<assets::DefaultFont>,
    chunks: Res<assets::SpriteChunks>,
) {
    commands.spawn((
        PresentBy,
        Text2dBundle {
            text: Text::from_section(
                "3187 presents...",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 80.0,
                    color: Color::LinearRgba(LinearRgba::WHITE),
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 14.37 + 2.1),
            ..Default::default()
        },
    ));
    commands.spawn((
        PresentBy,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(LOGICAL),
                color: Color::LinearRgba(LinearRgba::BLACK),
                ..Default::default()
            },
            texture: chunks.white.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 14.37 + 2.0),
            ..Default::default()
        },
    ));
    commands.insert_resource(PresentByTimer::default());
}

fn present_by_timer_tick(
    mut timer: ResMut<PresentByTimer>,
    time: Res<config::FrameTime>,
    mut state: ResMut<NextState<info::PlayStates>>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        state.set(info::PlayStates::Gaming);
    }
}

fn despawn_present_by(mut commands: Commands, q_present_by: Query<Entity, With<PresentBy>>) {
    q_present_by.iter().for_each(|entity| {
        if let Some(commands) = commands.get_entity(entity) {
            commands.despawn_recursive();
        }
    })
}

fn spawn_title(
    mut commands: Commands,
    chunks: Res<assets::SpriteChunks>,
    level: Res<level::Level>,
) {
    if level.config.game.contains(&level::GameKind::Intro) {
        commands.spawn((
            PresentBy,
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 14.27 + 2.0),
                sprite: Sprite {
                    custom_size: Some(LOGICAL / 2.0),
                    ..Default::default()
                },
                texture: chunks.pvfz.clone(),
                ..Default::default()
            },
        ));
        commands.insert_resource(TitleTimer::default());
    }
}

fn remove_banners(
    mut commands: Commands,
    q_banner: Query<Entity, With<level::Banner>>,
    level: Res<level::Level>,
) {
    if level.config.game.contains(&level::GameKind::Intro) {
        q_banner.iter().for_each(|entity| {
            if let Some(commands) = commands.get_entity(entity) {
                commands.despawn_recursive();
            }
        })
    }
}

fn title_timer_tick(
    mut timer: ResMut<TitleTimer>,
    time: Res<config::FrameTime>,
    mut state: ResMut<NextState<info::GlobalStates>>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        state.set(info::GlobalStates::Win);
    }
}
