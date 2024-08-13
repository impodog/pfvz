use crate::prelude::*;

pub(super) struct ModesQuickShotPlugin;

impl Plugin for ModesQuickShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (accelerate,).run_if(when_state!(gaming)));
    }
}

fn accelerate(level: Res<level::Level>, mut q_shooter: Query<&mut compn::ShooterImpl>) {
    if level.config.game == level::GameKind::QuickShot {
        q_shooter.par_iter_mut().for_each(|mut shooter| {
            if shooter.just_finished() {
                let mut duration = shooter.timer.duration();
                duration -= Duration::from_secs_f32(0.005);
                duration = duration.max(Duration::from_secs_f32(0.05));
                shooter.timer.set_duration(duration);
            }
        });
    }
}
