use super::*;
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            creature::GameCreaturePlugin,
            position::GamePositionPlugin,
            status::GameStatusPlugin,
            plant::GamePlantPlugin,
            proj::GameProjPlugin,
            zombie::GameZombiePlugin,
            spawn::GameSpawnPlugin,
        ));
    }
}
