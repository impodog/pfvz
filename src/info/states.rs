use crate::prelude::*;

pub(super) struct InfoStatesPlugin;

impl Plugin for InfoStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalStates>();
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalStates {
    #[default]
    Title,
    Menu,
    Play,
    Lose,
}
