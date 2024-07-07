use crate::prelude::*;

pub(super) struct LevelRoomPlugin;

impl Plugin for LevelRoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Room>();
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Room {
    pub cursor: usize,
}
