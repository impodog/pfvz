use crate::prelude::*;

pub(super) struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            |mut e_level: EventWriter<level::LevelEvent>, save: Res<save::Save>| {
                e_level.send(level::LevelEvent {
                    index: save.adventure.0,
                });
            },
        );
        app.add_systems(Update, (skip_level,).run_if(when_state!(play)));
    }
}

fn skip_level(mut state: ResMut<NextState<info::GlobalStates>>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::Enter) {
        state.set(info::GlobalStates::Win);
    }
}
