use crate::prelude::*;

pub(super) struct LevelRoomPlugin;

impl Plugin for LevelRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoomNextWave>();
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_room,));
        app.add_systems(Update, (update_room,).run_if(when_state!(gaming)));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RoomStatus {
    pub cursor: usize,
    pub timer: Timer,
    pub min_timer: Timer,
    pub fin: bool,
}
impl RoomStatus {
    pub fn with_timer(timer: Timer) -> Self {
        Self {
            timer,
            ..Default::default()
        }
    }
}
impl Default for RoomStatus {
    fn default() -> Self {
        Self {
            cursor: 0,
            timer: Timer::default(),
            min_timer: Timer::new(Duration::from_millis(5000), TimerMode::Once),
            fin: false,
        }
    }
}

#[derive(Event)]
pub struct RoomNextWave(pub usize);

fn init_room(mut commands: Commands, level: Res<level::Level>) {
    commands.insert_resource(RoomStatus::with_timer(Timer::new(
        Duration::from_secs_f32(level.waves.first().map(|wave| wave.wait).unwrap_or(0.0)),
        TimerMode::Once,
    )));
}

fn update_room(
    mut status: ResMut<RoomStatus>,
    level: Res<level::Level>,
    mut next_wave: EventWriter<RoomNextWave>,
    time: Res<config::FrameTime>,
    q_zombie: Query<(), (With<game::Zombie>, Without<game::NotInvasive>)>,
) {
    status.timer.tick(time.delta());
    status.min_timer.tick(time.delta());
    // A wave refreshes when all conditions meet:
    // 1. There is a pending wave
    // 2. The minimum interval of waves has passed
    // 3. The timer defined by user has finished, or when the zombies from the previous wave(if any) has been all killed
    if status.cursor < level.waves.len()
        && status.min_timer.finished()
        && (status.timer.finished() || (q_zombie.iter().next().is_none() && status.cursor > 0))
    {
        next_wave.send(RoomNextWave(status.cursor));
        info!("Updated to wave {}", status.cursor);
        status.timer = Timer::from_seconds(
            level.waves[(status.cursor + 1).min(level.waves.len() - 1)].wait,
            TimerMode::Once,
        );
        status.min_timer.reset();
        status.cursor += 1;

        if status.cursor >= level.waves.len() {
            status.fin = true;
        }
    }
}
