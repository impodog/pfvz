use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioPlants {
    pub fume_shroom: AudioList,
    pub spikeweed: AudioList,
    pub shooter: AudioList,
    pub spore: AudioList,
}

pub(super) fn load_plants(mut commands: Commands, server: Res<AssetServer>) {
    let plants = AudioPlants {
        fume_shroom: AudioList::load(&server, "audio/plants/fume_shroom"),
        spikeweed: AudioList::load(&server, "audio/plants/spikeweed"),
        shooter: AudioList::load(&server, "audio/plants/shooter"),
        spore: AudioList::load(&server, "audio/plants/spore"),
    };
    commands.insert_resource(plants);
}
