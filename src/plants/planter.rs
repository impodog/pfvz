use crate::prelude::*;

pub(super) struct PlantsPlanterPlugin;

impl Plugin for PlantsPlanterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlanterEvent>();
        app.add_systems(Update, (do_plant,).run_if(when_state!(gaming)));
    }
}

#[derive(Event, Debug, Clone)]
pub struct PlanterEvent {
    pub index: usize,
    pub id: Id,
}

#[allow(clippy::too_many_arguments)]
fn do_plant(
    mut commands: Commands,
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
    slots: Res<level::LevelSlots>,
    q_transform: Query<&Transform>,
) {
    if cursor.left && cursor.inbound {
        let coordinates = level.config.layout.position_to_coordinates(&cursor.pos);
        let pos = level.config.layout.regularize(cursor.pos);
        if let Some(id) = selection.get(select.0) {
            if let Some(creature) = map.get(id) {
                let ok = *id >= 0 || {
                    let index = level.config.layout.position_to_index(&cursor.pos);
                    plants
                        .plants
                        .get(index)
                        .is_some_and(|tile| tile.read().unwrap().is_empty())
                };
                if ok
                    && level
                        .config
                        .layout
                        .get_tile(coordinates.0, coordinates.1)
                        .compat(creature)
                    // NOTE: We may use `Option::is_none_or` if possible in the future
                    && !cooldown
                        .get_option(select.0)
                        .is_some_and(|timer| !timer.finished())
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
        } else if select.0 == slots.0 {
            let list = {
                let index = level.config.layout.position_to_index(&cursor.pos);
                plants.plants.get(index)
            };
            if let Some(list) = list {
                let list = list.read().unwrap();
                // This makes sure that only the top level of plant is removed
                let entity = list.iter().max_by(|left, right| {
                    if let Ok(left) = q_transform.get(**left) {
                        if let Ok(right) = q_transform.get(**right) {
                            return left
                                .translation
                                .z
                                .partial_cmp(&right.translation.z)
                                .unwrap();
                        }
                    }
                    std::cmp::Ordering::Less
                });
                if let Some(entity) = entity {
                    commands.entity(*entity).despawn_recursive();
                }
            }
            select.0 = usize::MAX;
        }
    }
}
