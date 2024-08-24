use crate::prelude::*;

pub(super) struct PlantsPlanterPlugin;

impl Plugin for PlantsPlanterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlanterEvent>();
        app.add_event::<PlanterCall>();
        app.add_systems(
            Update,
            (do_plant, receive_planter_call).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub struct PlanterEvent {
    pub index: usize,
    pub id: Id,
}

#[derive(Event, Default, Debug, Clone)]
pub struct PlanterCall {
    pub coordinates: (usize, usize),
    /// This is only used when the planter call is done by the user click
    /// You can safely set it to None if not needed
    pub selection_index: Option<usize>,
    pub id: Id,
}

fn receive_planter_call(
    mut call: EventReader<PlanterCall>,
    mut action: EventWriter<game::CreatureAction>,
    mut planter: EventWriter<PlanterEvent>,
    mut sun: ResMut<game::Sun>,
    select: ResMut<game::Selecting>,
    map: Res<game::CreatureMap>,
    plants: Res<game::PlantLayout>,
    cooldown: Res<game::SelectionCooldown>,
    level: Res<level::Level>,
    cursor: Res<info::CursorInfo>,
    q_creature: Query<&game::Creature>,
    q_pos: Query<&game::Position>,
    q_go_below: Query<(), With<game::PlantGoBelow>>,
) {
    for PlanterCall {
        coordinates,
        selection_index,
        id,
    } in call.read()
    {
        let pos = level
            .config
            .layout
            .coordinates_to_position(coordinates.0, coordinates.1);

        if let Some(creature) = map.get(id) {
            let index = level.config.layout.position_2d_to_index(&cursor.pos);

            let ok = *id >= 0 || {
                // Determines whether the plant is compatible with the tile selected
                // Or, when the tile is not empty, return whether the plant is compatible with
                // the top layer plant
                if let Some(plant) = plants.top(index) {
                    if let Ok(top_creature) = q_creature.get(plant) {
                        creature.flags.is_compat(top_creature.flags)
                    } else {
                        warn!("Top of a tile is not creature, this should not happen!");
                        true
                    }
                } else {
                    creature.flags.is_compat(
                        level
                            .config
                            .layout
                            .get_tile(coordinates.0, coordinates.1)
                            .flags(),
                    )
                }
            };
            if ok
                    // NOTE: We may use `Option::is_none_or` if possible in the future
                    && !cooldown
                        .get_option(select.0)
                        .is_some_and(|timer| !timer.finished())
                    && sun.0 >= creature.cost
            {
                // When planted on top, increase z height
                let mut disp = game::Position::default();
                if let Some(plant) = plants.top(index) {
                    if q_go_below.get(plant).is_err()
                        && creature.flags != level::CreatureFlags::PUMPKIN
                    {
                        if let Ok(top_pos) = q_pos.get(plant) {
                            disp.z += top_pos.z + SHADOW_DISTANCE
                                - level.config.layout.get_disp(coordinates.0);
                        }
                    }
                }
                let logic = if disp.z != 0.0 {
                    game::LogicPosition::from_base(pos).with_disp(disp)
                } else {
                    game::LogicPosition::from_base(pos)
                };
                action.send(game::CreatureAction::Spawn(*id, logic));
                // If selection is available
                if let Some(selection_index) = selection_index {
                    sun.0 -= creature.cost;
                    planter.send(PlanterEvent {
                        index: *selection_index,
                        id: *id,
                    });
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn do_plant(
    mut commands: Commands,
    mut select: ResMut<game::Selecting>,
    selection: Res<game::Selection>,
    plants: Res<game::PlantLayout>,
    level: Res<level::Level>,
    cursor: Res<info::CursorInfo>,
    slots: Res<level::LevelSlots>,
    q_transform: Query<&Transform>,
    q_creature: Query<&game::Creature>,
    mut call: EventWriter<PlanterCall>,
) {
    if cursor.left {
        if let Some(coordinates) = level
            .config
            .layout
            .position_2d_to_coordinates_checked(&cursor.pos)
        {
            if let Some(id) = selection.get(select.0) {
                call.send(PlanterCall {
                    coordinates,
                    selection_index: if select.0 < slots.0 {
                        Some(select.0)
                    } else {
                        None
                    },
                    id: *id,
                });
                select.0 = usize::MAX;
            } else if select.0 == slots.0 {
                let list = {
                    let index = level.config.layout.position_2d_to_index(&cursor.pos);
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
                        if let Ok(creature) = q_creature.get(*entity) {
                            // Filters undiggable creatures
                            if (creature.flags & level::CreatureFlags::UNDIGGABLE).bits() == 0 {
                                if let Some(commands) = commands.get_entity(*entity) {
                                    commands.despawn_recursive();
                                }
                            }
                        }
                    }
                }
                select.0 = usize::MAX;
            }
        }
    }
}
