use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioItems {
    pub whack: AudioList,
}

pub(super) fn load_items(mut commands: Commands, server: Res<AssetServer>) {
    let items = AudioItems {
        whack: AudioList::load(&server, "audio/items/whack"),
    };
    commands.insert_resource(items);
}
