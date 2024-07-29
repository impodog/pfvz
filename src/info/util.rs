#[macro_export]
macro_rules! when_state {
    (gaming) => {
        bevy::prelude::in_state($crate::info::PlayStates::Gaming)
    };
    (cys) => {
        bevy::prelude::in_state($crate::info::PlayStates::Cys)
    };
}
