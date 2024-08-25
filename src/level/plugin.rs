use super::*;
use crate::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            load::LevelLoadPlugin,
            room::LevelRoomPlugin,
            banners::LevelBannersPlugin,
            progress::LevelProgressPlugin,
            spawn::LevelSpawnPlugin,
            disp::LevelDispPlugin,
            bgm::LevelBgmPlugin,
        ));
    }
}
