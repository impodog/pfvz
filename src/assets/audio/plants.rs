use super::AudioList;
use crate::prelude::*;

#[derive(Resource)]
pub struct AudioPlants {
    pub fume_shroom: AudioList,
}

pub(super) fn load_plants(mut commands: Commands, server: Res<AssetServer>) {
    let plants = AudioPlants {
        fume_shroom: AudioList::load(&server, "audio/plants/fume_shroom"),
    };
    commands.insert_resource(plants);
}
