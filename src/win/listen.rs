use crate::prelude::*;

pub(super) struct WinListenPlugin;

impl Plugin for WinListenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Win), (update_save,));
        app.add_systems(OnExit(info::GlobalStates::Win), (despawn_win_stuff,));
        app.add_systems(
            Update,
            (listen_mouse).run_if(in_state(info::GlobalStates::Win)),
        );
    }
}

fn despawn_win_stuff(mut commands: Commands, q_pos: Query<Entity, With<game::Position>>) {
    q_pos.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

fn update_save(level: Res<level::Level>, mut save: ResMut<save::Save>) {
    if let Some(modify) = &level.config.modify {
        if modify.next > save.adventure.0 {
            save.adventure.0 = modify.next;
        }
    }
}

fn listen_mouse(
    mut e_level: EventWriter<level::LevelEvent>,
    mut state: ResMut<NextState<info::GlobalStates>>,
    cursor: Res<info::CursorInfo>,
    level: Res<level::Level>,
    q_banner: Query<(), With<level::Banner>>,
) {
    // Wait for banner to disappear
    if q_banner.iter().next().is_none() {
        if cursor.left {
            if let Some(modify) = &level.config.modify {
                e_level.send(level::LevelEvent { index: modify.next });
            } else {
                state.set(info::GlobalStates::Menu);
            }
        } else if cursor.right {
            state.set(info::GlobalStates::Menu);
        }
    }
}
