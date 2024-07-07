use crate::prelude::*;

pub(super) struct GameSpawnPlugin;

impl Plugin for GameSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(info::GlobalStates::Play), (despawn_game_items,));
    }
}

#[derive(Component)]
pub struct GameItem;

fn despawn_game_items(mut commands: Commands, q_item: Query<Entity, With<GameItem>>) {
    q_item.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
