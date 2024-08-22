use crate::prelude::*;

pub(super) struct ModesThunderPlugin;

impl Plugin for ModesThunderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FlashEvent>();
        app.add_systems(OnEnter(info::GlobalStates::Play), (spawn_thunder,));
        app.add_systems(
            Update,
            (modify_thunder_color, flash_timer_tick, flash_impl).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component)]
pub struct ThunderMarker;

#[derive(Resource, Debug)]
pub struct ThunderColor(pub f32, pub f32);

impl Default for ThunderColor {
    fn default() -> Self {
        Self(0.0, 1.0)
    }
}

fn spawn_thunder(
    mut commands: Commands,
    display: Res<game::Display>,
    level: Res<level::Level>,
    chunks: Res<assets::SpriteChunks>,
) {
    commands.insert_resource(ThunderColor::default());
    commands.insert_resource(FlashPeriod::default());
    if level.config.game.contains(&level::GameKind::Thunder) {
        let rows = (LOGICAL_WIDTH / display.ratio) as i32 + 1;
        let cols = (LOGICAL_HEIGHT / display.ratio) as i32 + 1;
        let half_rows = rows / 2;
        let half_cols = cols / 2;
        let size = level.config.layout.half_size_f32();
        let base = level
            .config
            .layout
            .coordinates_to_position(size.0 as usize, size.1 as usize);
        for x in -half_rows..=half_rows {
            for y in -half_cols..=half_cols {
                let pos = base + game::Position::new_xy(x as f32, y as f32);
                commands.spawn((
                    modes::FogMarker,
                    ThunderMarker,
                    modes::FogBloverImmunity,
                    pos,
                    game::HitBox::new(1.0, 1.0),
                    SpriteBundle {
                        texture: chunks.white.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 14.37 - 0.5),
                        sprite: Sprite {
                            color: Color::LinearRgba(LinearRgba::BLACK),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

fn modify_thunder_color(
    mut q_thunder: Query<&mut Sprite, With<ThunderMarker>>,
    color: Res<ThunderColor>,
) {
    q_thunder.par_iter_mut().for_each(|mut sprite| {
        sprite.color = Color::LinearRgba(LinearRgba::new(color.0, color.0, color.0, color.1));
    });
}

#[derive(Event)]
pub struct FlashEvent;

#[derive(Debug, Deref, DerefMut)]
struct FlashingTimer(Timer);
impl Default for FlashingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

fn flash_timer_tick(
    time: Res<config::FrameTime>,
    mut timer: Local<FlashingTimer>,
    factors: Res<collectible::ItemFactors>,
    mut flash: EventWriter<FlashEvent>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        flash.send(FlashEvent);
        timer.set_duration(factors.thunder.interval.into());
        timer.reset();
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub enum FlashPeriod {
    #[default]
    Paused,
    ToWhite,
    ToDark,
    ToOpaque,
}

#[derive(Debug, Deref, DerefMut)]
struct FlashModifyTimer(Timer);
impl Default for FlashModifyTimer {
    fn default() -> Self {
        FlashModifyTimer(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}

fn flash_impl(
    mut color: ResMut<ThunderColor>,
    mut period: ResMut<FlashPeriod>,
    mut timer: Local<FlashModifyTimer>,
    time: Res<config::FrameTime>,
    mut e_flash: EventReader<FlashEvent>,
) {
    if e_flash.read().next().is_some() {
        *period = FlashPeriod::ToWhite;
    }
    timer.tick(time.delta());
    if timer.just_finished() {
        match *period {
            FlashPeriod::Paused => {}
            FlashPeriod::ToWhite => {
                if color.0 < 0.5 {
                    color.0 = (color.0 + 0.08).min(1.0);
                } else {
                    *period = FlashPeriod::ToDark;
                }
            }
            FlashPeriod::ToDark => {
                if color.0 > 0.0 {
                    color.0 = (color.0 - 0.05).max(0.0);
                    color.1 = (color.1 - 0.1).max(0.0);
                } else {
                    *period = FlashPeriod::ToOpaque;
                }
            }
            FlashPeriod::ToOpaque => {
                if color.1 < 1.0 {
                    color.1 = (color.1 + 0.05).min(1.0);
                } else {
                    *period = FlashPeriod::Paused;
                }
            }
        }
    }
}
