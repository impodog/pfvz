use super::*;
use crate::prelude::*;

pub struct InfoPlugin;

impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            mouse::InfoMousePlugin,
            states::InfoStatesPlugin,
            pause::InfoPausePlugin,
        ));
    }
}
