use super::*;
use crate::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::ConfigWindowPlugin,
            camera::ConfigCameraPlugin,
            load::ConfigLoadPlugin,
            program::ConfigProgramPlugin,
        ));
    }
}
