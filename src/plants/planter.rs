use crate::prelude::*;

pub(super) struct PlantsPlanterPlugin;

impl Plugin for PlantsPlanterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlanterEvent>();
        app.add_systems(
            Update,
            (do_plant,).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub struct PlanterEvent {
    pub index: usize,
    pub id: Id,
}

#[allow(clippy::too_many_arguments)]
fn do_plant(
    mut action: EventWriter<game::CreatureAction>,
    mut planter: EventWriter<PlanterEvent>,
    mut sun: ResMut<game::Sun>,
    mut select: ResMut<game::Selecting>,
    selection: Res<game::Selection>,
    map: Res<game::CreatureMap>,
    plants: Res<game::PlantLayout>,
    cooldown: Res<game::SelectionCooldown>,
    level: Res<level::Level>,
    cursor: Res<info::CursorInfo>,
) {
    if cursor.left && cursor.inbound {
        if let Some(id) = selection.get(select.0) {
            if let Some(creature) = map.get(id) {
                let pos = cursor.pos.regularize();
                let usize_pos = cursor.pos.to_usize_pos();
                let ok = *id >= 0 || {
                    let index = level.config.layout.position_to_index(&pos);
                    plants
                        .plants
                        .get(index)
                        .is_some_and(|tile| tile.read().unwrap().is_empty())
                };
                if ok
                    && level
                        .config
                        .layout
                        .get_tile(usize_pos.0, usize_pos.1)
                        .compat(creature)
                    && cooldown
                        .get_option(select.0)
                        .is_none_or(|timer| timer.finished())
                    && sun.0 >= creature.cost
                {
                    sun.0 -= creature.cost;
                    action.send(game::CreatureAction::Spawn(*id, pos));
                    planter.send(PlanterEvent {
                        index: select.0,
                        id: *id,
                    });
                    select.0 = usize::MAX;
                }
            }
        }
    }
}
