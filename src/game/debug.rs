use crate::prelude::*;

pub(super) struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (skip_level,).run_if(when_state!(play)));
    }
}

fn skip_level(
    mut state: ResMut<NextState<info::GlobalStates>>,
    key: Res<ButtonInput<KeyCode>>,
    level_index: Res<level::LevelIndex>,
) {
    if key.just_pressed(KeyCode::Enter) && level_index.stage == 0 {
        state.set(info::GlobalStates::Win);
    }
}
