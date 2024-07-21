use crate::prelude::*;

pub(super) struct PlantsPlanterPlugin;

impl Plugin for PlantsPlanterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (do_plant,));
    }
}

fn do_plant(
    mut action: EventWriter<game::CreatureAction>,
    mut sun: ResMut<game::Sun>,
    select: Res<game::Selecting>,
    selection: Res<game::Selection>,
    map: Res<game::CreatureMap>,
    cursor: Res<info::CursorInfo>,
) {
    if cursor.left && cursor.inbound {
        if let Some(id) = selection.get(select.0) {
            if let Some(creature) = map.get(id) {
                if sun.0 >= creature.cost {
                    sun.0 -= creature.cost;
                    action.send(game::CreatureAction::Spawn(*id, cursor.pos));
                }
            }
        }
    }
}
