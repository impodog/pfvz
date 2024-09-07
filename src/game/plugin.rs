use super::*;
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Put this in the front of this set may reduce bugs
            overlay::GameOverlayPlugin,
            creature::GameCreaturePlugin,
            position::GamePositionPlugin,
            velocity::GameVelocityPlugin,
            status::GameStatusPlugin,
            plant::GamePlantPlugin,
            proj::GameProjPlugin,
            zombie::GameZombiePlugin,
            spawn::GameSpawnPlugin,
            player::GamePlayerPlugin,
            debug::GameDebugPlugin,
            size::GameSizePlugin,
            pos_util::GamePosUtilPlugin,
        ));
    }
}
