use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioZombies {
    pub bite: AudioList,
}

pub(super) fn load_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let zombies = AudioZombies {
        bite: AudioList::load(&server, "audio/zombies/bite"),
    };
    commands.insert_resource(zombies);
}
