use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioZombies {
    pub bite: AudioList,
    pub water: AudioList,
    pub zomboni: AudioList,
    pub jitb: AudioList,
    pub explode: AudioList,
}

pub(super) fn load_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let zombies = AudioZombies {
        bite: AudioList::load(&server, "audio/zombies/bite"),
        water: AudioList::load(&server, "audio/zombies/water"),
        zomboni: AudioList::load(&server, "audio/zombies/zomboni"),
        jitb: AudioList::load(&server, "audio/zombies/jitb"),
        explode: AudioList::load(&server, "audio/zombies/explode"),
    };
    commands.insert_resource(zombies);
}
