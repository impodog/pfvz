use super::*;
use crate::prelude::*;

pub struct AchPlugin;

impl Plugin for AchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            load::AchLoadPlugin,
            update::AchUpdatePlugin,
            listen::AchListenPlugin,
        ));
    }
}
