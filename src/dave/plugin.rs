use crate::prelude::*;

pub struct DavePlugin;

impl Plugin for DavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((dave::DaveDavePlugin,));
    }
}
