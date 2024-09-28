use crate::prelude::*;

pub(super) struct AchWinningPlugin;

impl Plugin for AchWinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::GlobalStates::Win),
            (listen_fungi_party, listen_dark_rules, listen_who_cares),
        );
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_collectible_count,));
        app.add_systems(
            Update,
            (test_collectible_spawn, listen_loss).run_if(when_state!(gaming)),
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

#[derive(Resource, Default)]
pub struct SunCount(pub usize);

fn init_collectible_count(mut commands: Commands) {
    commands.insert_resource(SunCount::default());
    commands.insert_resource(WhoCaresFlag::default());
}

fn test_collectible_spawn(
    mut count: ResMut<SunCount>,
    q_sun: Query<(), Added<collectible::Collectible>>,
    mut e_coll: EventReader<collectible::CollectibleEvent>,
) {
    count.0 += q_sun.iter().count();
    count.0 = count.0.saturating_sub(e_coll.read().count());
}

#[derive(Resource)]
pub struct WhoCaresFlag(pub bool);
impl Default for WhoCaresFlag {
    fn default() -> Self {
        Self(true)
    }
}

fn listen_loss(
    mut e_action: EventReader<game::CreatureAction>,
    q_creature: Query<&game::Creature>,
    flag: ResMut<WhoCaresFlag>,
) {
    if !flag.0 {
        return;
    }
    let flag = Mutex::new(flag);
    e_action.par_read().for_each(|action| {
        if let game::CreatureAction::Die(entity) = action {
            if let Ok(creature) = q_creature.get(*entity) {
                if IdFeature::from(creature.id) == IdFeature::Fungi {
                    flag.lock().unwrap().0 = false;
                }
            }
        }
    });
}

fn listen_who_cares(
    mut e_ach: EventWriter<ach::NewAchievement>,
    index: Res<level::LevelIndex>,
    count: Res<SunCount>,
    flag: Res<WhoCaresFlag>,
) {
    if matches!(
        *index,
        level::LevelIndex {
            stage: 2,
            level: 15
        }
    ) && count.0 == 0
        && flag.0
    {
        e_ach.send(ach::NewAchievement(ach::AchId::WhoCares));
    }
}
