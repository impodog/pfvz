use crate::prelude::*;

pub(super) struct ConfigUtilPlugin;

impl Plugin for ConfigUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (init_timer,));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct FrameTime {
    delta: Duration,
}
impl FrameTime {
    pub fn delta(&self) -> Duration {
        self.delta
    }
}

fn init_timer(mut commands: Commands, config: Res<config::Config>) {
    commands.insert_resource(FrameTime {
        delta: Duration::from_millis(
            (1000.0 / config.program.framerate.0 * config.gamerule.speed.0) as u64,
        ),
    })
}
