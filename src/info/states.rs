use crate::prelude::*;

pub(super) struct InfoStatesPlugin;

impl Plugin for InfoStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalStates>();
        app.init_state::<PlayStates>();
        app.init_state::<MenuStates>();
        app.add_systems(OnEnter(GlobalStates::Play), (change_to_dave,));
        app.add_systems(OnExit(GlobalStates::Play), (change_to_not_playing,));
        app.add_systems(OnEnter(GlobalStates::Menu), (change_to_main_menu,));
        app.add_systems(OnExit(GlobalStates::Menu), (change_to_not_menu,));
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalStates {
    #[default]
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
    Intro,
    Gaming,
}

fn change_to_dave(mut play: ResMut<NextState<PlayStates>>) {
    play.set(PlayStates::Dave);
}

fn change_to_not_playing(mut play: ResMut<NextState<PlayStates>>) {
    play.set(PlayStates::NotPlaying);
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuStates {
    #[default]
    NotMenu,
    Main,
    Adventure,
}

fn change_to_main_menu(mut menu: ResMut<NextState<MenuStates>>) {
    menu.set(MenuStates::Main);
}

fn change_to_not_menu(mut menu: ResMut<NextState<MenuStates>>) {
    menu.set(MenuStates::NotMenu);
}
