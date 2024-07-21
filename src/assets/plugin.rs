use super::*;
use crate::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((sprites::AssetsSpritesPlugin, fonts::AssetsFontsPlugin));
    }
}
