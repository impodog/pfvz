use super::*;
use crate::prelude::*;

pub struct AlmanacPlugin;

impl Plugin for AlmanacPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AlmanacShowPlugin,
            AlmanacStatesPlugin,
            AlmanacMenuPlugin,
            AlmanacCreaturePlugin,
        ));
    }
}
