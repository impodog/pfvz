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
    // Multiply this in position manipulation systems
    // to get ideal velocity for every possibility of framerate
    diff: f32,
    paused: bool,
}
impl FrameTime {
    pub fn delta(&self) -> Duration {
        if self.paused {
            Duration::default()
        } else {
            self.delta
        }
    }

    /// The speed factor to multiply
    /// This is only affected by the framerate users sets
    pub fn diff(&self) -> f32 {
        if self.paused {
            0.0
        } else {
            self.diff
        }
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }
}

fn init_timer(mut commands: Commands, config: Res<config::Config>) {
    let diff = 1.0 / config.program.framerate.0 * config.gamerule.speed.0;
    commands.insert_resource(FrameTime {
        delta: Duration::from_secs_f32(diff),
        diff,
        paused: false,
    })
}
