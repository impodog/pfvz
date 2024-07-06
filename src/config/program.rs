use crate::prelude::*;

pub(super) struct ConfigProgramPlugin;

impl Plugin for ConfigProgramPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (control_framerate,));
    }
}

fn control_framerate(time: Res<Time>, config: Res<super::Config>) {
    let secs = time.delta_seconds();
    let target = 1.0 / config.program.framerate.0;
    if secs < target {
        std::thread::sleep(std::time::Duration::from_secs_f32(target - secs));
    }
}
