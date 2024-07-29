use crate::prelude::*;

pub(super) struct ChooseExitPlugin;

impl Plugin for ChooseExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(info::PlayStates::Cys), (despawn_selection,));
    }
}

fn despawn_selection(mut commands: Commands, q_sel: Query<Entity, With<choose::SelectionMarker>>) {
    q_sel.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
