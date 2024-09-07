use crate::prelude::*;

pub(super) struct AchListenPlugin;

impl Plugin for AchListenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (listen_getting_exciting,).run_if(when_state!(gaming)),
        );
    }
}

fn listen_getting_exciting(
    mut e_ach: EventWriter<ach::NewAchievement>,
    bgm: Res<level::BgmStatus>,
) {
    if bgm.is_changed() && *bgm == level::BgmStatus::Exciting {
        e_ach.send(ach::NewAchievement(ach::AchId::GettingExciting));
    }
}
