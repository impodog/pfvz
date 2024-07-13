use super::*;
use crate::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((data::SaveDataPlugin,));
    }
}
