use super::*;
use crate::prelude::*;

pub struct LosePlugin;

impl Plugin for LosePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((banner::LoseBannerPlugin, listen::LoseListenPlugin));
    }
}
