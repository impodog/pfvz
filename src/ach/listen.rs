use crate::prelude::*;

pub(super) struct AchListenPlugin;

impl Plugin for AchListenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (listen_getting_exciting, listen_impulsive_killer).run_if(when_state!(gaming)),
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

fn listen_impulsive_killer(
    mut e_ach: EventWriter<ach::NewAchievement>,
    mut e_action: EventReader<game::CreatureAction>,
    q_all_star: Query<&zombies::AllStarZombieRunning>,
) {
    let ok = Mutex::new(false);
    e_action.par_read().for_each(|action| {
        if let game::CreatureAction::Die(entity) = action {
            if q_all_star.get(*entity).is_ok_and(|running| running.0) {
                *ok.lock().unwrap() = true;
            }
        }
    });
    if Mutex::into_inner(ok).unwrap() {
        e_ach.send(ach::NewAchievement(ach::AchId::ImpulsiveKiller));
    }
}
