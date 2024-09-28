use super::*;
use crate::prelude::*;

pub struct ExZombiesPlugin;

impl Plugin for ExZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExZombiesFactorsPlugin,
            ExZombiesRallyPlugin,
            ExZombiesBrickPlugin,
            ExZombiesGigaPlugin,
            ExZombiesSuneditPlugin,
            ExZombiesMirrorPlugin,
        ));
    }
}
