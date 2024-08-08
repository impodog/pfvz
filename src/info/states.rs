use crate::prelude::*;

pub(super) struct InfoStatesPlugin;

impl Plugin for InfoStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalStates>();
        app.init_state::<PlayStates>();
        app.add_systems(OnEnter(GlobalStates::Play), (change_to_dave,));
        app.add_systems(OnExit(GlobalStates::Play), (change_to_not_playing,));
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalStates {
    #[default]
    Title,
    Menu,
    Play,
    Lose,
    Win,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayStates {
    #[default]
    NotPlaying,
    Dave,
    Cys,
    Gaming,
}

fn change_to_dave(mut play: ResMut<NextState<PlayStates>>) {
    play.set(PlayStates::Dave);
}

fn change_to_not_playing(mut play: ResMut<NextState<PlayStates>>) {
    play.set(PlayStates::NotPlaying);
}
