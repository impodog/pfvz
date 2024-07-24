use crate::prelude::*;

pub(super) struct GameZombiePlugin;

impl Plugin for GameZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_win_timer,));
        app.add_systems(
            PostUpdate,
            (losing_test, winning_test).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zombie;

#[derive(Component, Debug)]
pub struct ZombieRelevant;

#[derive(Component, Debug)]
pub struct NotInvasive;

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
struct WinTimer(Timer);

fn init_win_timer(mut commands: Commands) {
    commands.insert_resource(WinTimer(Timer::new(
        Duration::from_secs_f32(3.0),
        TimerMode::Once,
    )));
}

fn losing_test(
    mut state: ResMut<NextState<info::GlobalStates>>,
    q_zombie: Query<&game::Position, (With<Zombie>, Without<NotInvasive>)>,
    level: Res<level::Level>,
) {
    let ok = RwLock::new(false);
    let size = level.config.layout.size();
    q_zombie.par_iter().for_each(|pos| {
        if pos.x <= -(size.0 as f32 / 2.0) - 0.5 {
            *ok.write().unwrap() = true;
        }
    });
    if RwLock::into_inner(ok).unwrap() {
        state.set(info::GlobalStates::Lose);
    }
}

fn winning_test(
    mut state: ResMut<NextState<info::GlobalStates>>,
    q_zombie: Query<(), With<Zombie>>,
    level: Res<level::Level>,
    status: Res<level::RoomStatus>,
    mut win_timer: ResMut<WinTimer>,
    time: Res<config::FrameTime>,
) {
    if q_zombie.iter().next().is_none() && status.cursor >= level.waves.len() {
        win_timer.tick(time.delta());
        if win_timer.just_finished() {
            state.set(info::GlobalStates::Win);
        }
    }
}
