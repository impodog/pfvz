use crate::prelude::*;

pub(super) struct ConfigProgramPlugin;

impl Plugin for ConfigProgramPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (control_framerate, quit));
    }
}

fn control_framerate(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    config: Res<super::Config>,
) {
    settings.limiter = bevy_framepace::Limiter::from_framerate(config.program.framerate.0 as f64);
}

fn quit(mut state: ResMut<NextState<info::GlobalStates>>, button: Res<ButtonInput<KeyCode>>) {
    if button.just_pressed(KeyCode::Escape) {
        state.set(info::GlobalStates::Menu);
    }
}
