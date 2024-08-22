use crate::prelude::*;

pub(super) struct ModesRandomPlugin;

impl Plugin for ModesRandomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_chances,));
        app.add_systems(Update, (random_choose,).run_if(when_state!(gaming)));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct RandomChances(BTreeMap<Id, usize>);

fn init_chances(mut commands: Commands, save: Res<save::Save>) {
    let map = BTreeMap::from_iter(save.plants.iter().cloned().map(|id| (id, 10)));
    commands.insert_resource(RandomChances(map));
}

fn random_choose(
    mut selection: ResMut<game::Selection>,
    mut e_show: EventWriter<game::ShowSelectionEvent>,
    mut planter: EventReader<plants::PlanterEvent>,
    save: Res<save::Save>,
    level: Res<level::Level>,
    mut chances: ResMut<RandomChances>,
) {
    if level.config.game.contains(&level::GameKind::Random) {
        let ok = planter.read().any(|event| {
            let plants::PlanterEvent { index, id: _id } = *event;
            if index == 0 || index == 1 {
                false
            } else {
                let items = save
                    .plants
                    .iter()
                    .cloned()
                    .filter(|id| level.config.is_compat(*id))
                    .collect::<Vec<_>>();
                let id = items
                    .choose_weighted(&mut rand::thread_rng(), |id| {
                        chances.get(id).cloned().unwrap_or_default()
                    })
                    .cloned()
                    .unwrap_or(-1);
                selection.0[index] = id;
                if let Some(chance) = chances.get_mut(&id) {
                    *chance /= 2;
                    *chance = (*chance).max(2);
                }

                true
            }
        });
        if ok {
            e_show.send(game::ShowSelectionEvent);
        }
    }
}
