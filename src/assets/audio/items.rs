use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioItems {
    pub whack: AudioList,
    pub sun: AudioList,
    pub thunder: AudioList,
    pub intro: AudioList,
}

pub(super) fn load_items(mut commands: Commands, server: Res<AssetServer>) {
    let items = AudioItems {
        whack: AudioList::load(&server, "audio/items/whack"),
        sun: AudioList::load(&server, "audio/items/sun"),
        thunder: AudioList::load(&server, "audio/items/thunder"),
        intro: AudioList::load(&server, "audio/items/intro"),
    };
    commands.insert_resource(items);
}
