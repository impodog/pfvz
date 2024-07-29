use super::*;
use crate::prelude::*;

pub struct ChoosePlugin;

impl Plugin for ChoosePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((show::ChooseShowPlugin, exit::ChooseExitPlugin));
    }
}
