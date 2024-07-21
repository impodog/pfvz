use super::*;
use crate::prelude::*;

pub struct CollectiblePlugin;

impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            coll::CollectibleCollPlugin,
            factors::CollectibleFactorsPlugin,
            collect::CollectibleCollectPlugin,
            spawn::CollectibleSpawnPlugin,
        ));
    }
}
