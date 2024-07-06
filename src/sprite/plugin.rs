use super::*;
use crate::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((animation::SpriteAnimationPlugin,));
    }
}
