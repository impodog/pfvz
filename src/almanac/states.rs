use crate::prelude::*;

pub(super) struct AlmanacStatesPlugin;

impl Plugin for AlmanacStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AlmanacStates>();
        app.add_systems(OnEnter(info::MenuStates::Almanac), (change_to_menu,));
        app.add_systems(OnExit(info::MenuStates::Almanac), (change_to_not_almanac,));
        app.add_systems(Update, (exit_by_key,).run_if(when_state!(almanac)));
    }
}

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlmanacStates {
    #[default]
    NotAlmanac,
    Menu,
    Creature,
}

fn change_to_not_almanac(mut state: ResMut<NextState<AlmanacStates>>) {
    state.set(AlmanacStates::NotAlmanac);
}

fn change_to_menu(mut state: ResMut<NextState<AlmanacStates>>) {
    state.set(AlmanacStates::Menu);
}

fn exit_by_key(
    key: Res<ButtonInput<KeyCode>>,
    cursor: Res<info::CursorInfo>,
    almanac_state: Res<State<AlmanacStates>>,
    mut almanac: ResMut<NextState<AlmanacStates>>,
    mut menu: ResMut<NextState<info::MenuStates>>,
    mut page: ResMut<almanac::AlmanacPage>,
) {
    let ok = key.just_pressed(KeyCode::Escape) || cursor.right;
    if ok {
        if !matches!(almanac_state.get(), AlmanacStates::Menu) {
            almanac.set(AlmanacStates::Menu);
            page.0 = 0;
        } else {
            menu.set(info::MenuStates::Main);
        }
    }
}
