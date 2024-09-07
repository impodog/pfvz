use crate::prelude::*;

pub(super) struct ChooseExitPlugin;

impl Plugin for ChooseExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(info::PlayStates::Cys),
            (despawn_selection, save_selection),
        );
    }
}

fn save_selection(
    sel: Res<game::Selection>,
    mut save: ResMut<save::Save>,
    level: Res<level::Level>,
) {
    if let level::SelectionArr::Any = &level.config.selection {
        save.selection.0.clone_from(&sel.0);
    }
}

fn despawn_selection(mut commands: Commands, q_sel: Query<Entity, With<choose::SelectionMarker>>) {
    q_sel.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
