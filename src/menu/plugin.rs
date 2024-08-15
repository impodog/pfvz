use super::*;
use crate::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((main::MenuMainPlugin, adventure::MenuAdventurePlugin));
    }
}
