use crate::prelude::*;

pub(super) struct LoseListenPlugin;

impl Plugin for LoseListenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_mouse,).run_if(in_state(info::GlobalStates::Lose)),
        );
    }
}

fn listen_mouse(
    mut e_level: EventWriter<level::LevelIndex>,
    mut state: ResMut<NextState<info::GlobalStates>>,
    cursor: Res<info::CursorInfo>,
    level_index: Res<level::LevelIndex>,
    q_banner: Query<(), With<level::Banner>>,
) {
    // Wait for banner to disappear
    if q_banner.iter().next().is_none() {
        if cursor.left {
            e_level.send(*level_index);
        } else if cursor.right {
            state.set(info::GlobalStates::Menu);
        }
    }
}
