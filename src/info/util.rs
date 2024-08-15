#[macro_export]
macro_rules! when_state {
    (gaming) => {
        bevy::prelude::in_state($crate::info::PlayStates::Gaming)
    };
    (cys) => {
        bevy::prelude::in_state($crate::info::PlayStates::Cys)
    };
    (dave) => {
        bevy::prelude::in_state($crate::info::PlayStates::Dave)
    };
    (play) => {
        bevy::prelude::in_state($crate::info::GlobalStates::Play)
    };
    (main) => {
        bevy::prelude::in_state($crate::info::MenuStates::Main)
    };
    (adventure) => {
        bevy::prelude::in_state($crate::info::MenuStates::Adventure)
    };
    (menu) => {
        bevy::prelude::in_state($crate::info::GlobalStates::Menu)
    };
}
