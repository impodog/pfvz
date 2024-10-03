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
    (intro) => {
        bevy::prelude::in_state($crate::info::PlayStates::Intro)
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
    (config) => {
        bevy::prelude::in_state($crate::info::MenuStates::Config)
    };
    (menu) => {
        bevy::prelude::in_state($crate::info::GlobalStates::Menu)
    };
    (credits) => {
        bevy::prelude::in_state($crate::info::MenuStates::Credits)
    };
    (achievements) => {
        bevy::prelude::in_state($crate::info::MenuStates::Achievements)
    };
    (almanac) => {
        bevy::prelude::in_state($crate::info::MenuStates::Almanac)
    };
    (almanac_menu) => {
        bevy::prelude::in_state($crate::almanac::AlmanacStates::Menu)
    };
    (almanac_creature) => {
        bevy::prelude::in_state($crate::almanac::AlmanacStates::Creature)
    };
}
