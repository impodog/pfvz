use crate::prelude::*;

pub(super) struct AchWinningPlugin;

impl Plugin for AchWinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::GlobalStates::Win),
            (listen_fungi_party, listen_dark_rules),
        );
    }
}

fn listen_fungi_party(
    mut e_ach: EventWriter<ach::NewAchievement>,
    selection: Res<game::Selection>,
    level: Res<level::Level>,
) {
    if !level.config.layout.is_night() {
        let ok = !selection.is_empty()
            && selection.iter().all(|id| {
                *id == COFFEE_BEAN
                    || matches!(
                        IdFeature::from(*id),
                        IdFeature::Nonplantae | IdFeature::Fungi
                    )
            });
        if ok {
            e_ach.send(ach::NewAchievement(ach::AchId::FungiParty));
        }
    }
}

fn listen_dark_rules(
    mut e_ach: EventWriter<ach::NewAchievement>,
    selection: Res<game::Selection>,
    index: Res<level::LevelIndex>,
) {
    if matches!(
        *index,
        level::LevelIndex {
            stage: 4,
            level: 10
        }
    ) {
        let ok = !selection.is_empty() && selection.iter().all(|id| !matches!(*id, PLANTERN));
        if ok {
            e_ach.send(ach::NewAchievement(ach::AchId::DarkRules));
        }
    }
}
