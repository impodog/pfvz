use super::*;
use crate::prelude::*;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)));
        app.add_plugins((animation::SpriteAnimationPlugin, layout::SpriteLayoutPlugin));
    }
}
