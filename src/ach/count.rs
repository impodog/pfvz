use crate::prelude::*;

pub(super) struct AchCountPlugin;

impl Plugin for AchCountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (start_speedrun,));
        app.add_systems(OnEnter(info::GlobalStates::Win), (listen_speedrun_101,));
    }
}

#[derive(Resource, Debug)]
pub struct LevelSpeedrunTimer {
    begin: std::time::SystemTime,
    expected: Duration,
}

fn start_speedrun(mut commands: Commands, level: Res<level::Level>) {
    let time = level.waves.iter().fold(0.0, |acc, wave| acc + wave.wait);
    commands.insert_resource(LevelSpeedrunTimer {
        begin: std::time::SystemTime::now(),
        expected: Duration::from_secs_f32(time),
    });
}

fn listen_speedrun_101(
    mut e_ach: EventWriter<ach::NewAchievement>,
    speedrun: Option<Res<LevelSpeedrunTimer>>,
) {
    if let Some(speedrun) = speedrun {
        let duration = std::time::SystemTime::now()
            .duration_since(speedrun.begin)
            .unwrap_or_default();
        if duration <= speedrun.expected.mul_f32(2.0 / 3.0) {
            e_ach.send(ach::NewAchievement(ach::AchId::SpeedRun101));
        }
    }
}
