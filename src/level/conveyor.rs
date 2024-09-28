use crate::prelude::*;

pub(super) struct LevelConveyorPlugin;

impl Plugin for LevelConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConveyorNewCall>();
        app.add_systems(OnEnter(info::PlayStates::Cys), (close_cys,));
        app.add_systems(OnEnter(info::PlayStates::Gaming), (setup_conveyor,));
        app.add_systems(
            Update,
            (
                spawn_new_slot_test,
                (conveyor_displace, spawn_new_slot).chain(),
                conveyor_used,
            )
                .run_if(when_state!(gaming)),
        );
        app.add_systems(Last, (highlighter_displace,).run_if(when_state!(gaming)));
    }
}

fn close_cys(level: Res<level::Level>, mut state: ResMut<NextState<info::PlayStates>>) {
    if level.conveyor.is_some() {
        state.set(info::PlayStates::Gaming)
    }
}

fn setup_conveyor(mut commands: Commands, level: Res<level::Level>) {
    if let Some(ref conveyor) = level.conveyor {
        commands.insert_resource(ConveyorTimer(Timer::from_seconds(
            conveyor.interval,
            TimerMode::Repeating,
        )));
        commands.insert_resource(ConveyorDisplace(0.0));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ConveyorTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct ConveyorDisplace(pub f32);

#[derive(Event)]
pub struct ConveyorNewCall;

fn spawn_new_slot_test(
    mut timer: ResMut<ConveyorTimer>,
    time: Res<config::FrameTime>,
    mut selection: ResMut<game::Selection>,
    level: Res<level::Level>,
    mut call: EventWriter<ConveyorNewCall>,
) {
    if level.conveyor.is_some() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let len = level.config.selection.len();
            if selection.len() < len {
                selection.resize(len, 0);
            }
            call.send(ConveyorNewCall);
        }
    }
}

fn spawn_new_slot(
    mut timer: ResMut<ConveyorTimer>,
    mut call: EventReader<ConveyorNewCall>,
    mut selection: ResMut<game::Selection>,
    level: Res<level::Level>,
    q_creature: Query<&game::Creature>,
    displace: Res<ConveyorDisplace>,
    mut e_selection: EventWriter<game::ShowSelectionEvent>,
) {
    if call.read().next().is_none() {
        return;
    }
    if selection.last().is_some_and(|id| *id != 0) {
        return;
    }
    if displace.0 != 0.0 {
        let duration = timer.duration();
        timer.set_elapsed(duration);
        return;
    }
    if let Some(ref conveyor) = level.conveyor {
        let mut count = BTreeMap::new();
        count.extend(conveyor.items.keys().map(|key| (*key, Mutex::new(0usize))));
        let add_id = |id| {
            if let Some(count) = count.get(&id) {
                *count.lock().unwrap() += 1;
            }
        };
        q_creature.par_iter().for_each(|creature| {
            add_id(creature.id);
        });
        selection.iter().copied().for_each(add_id);
        let items = conveyor
            .items
            .iter()
            .filter_map(|(id, item)| {
                count.remove(id).map(|count| {
                    let count = Mutex::into_inner(count).unwrap();
                    let weight =
                        item.weight * (item.max.saturating_sub(count)) as f32 / item.max as f32;
                    (*id, weight)
                })
            })
            .collect::<Vec<_>>();
        let id = items.choose_weighted(&mut rand::thread_rng(), |(_, weight)| *weight);
        if let Ok((id, _)) = id {
            if let Some(last) = selection.last_mut() {
                *last = *id;
                e_selection.send(game::ShowSelectionEvent);
            }
        }
    }
}

fn conveyor_displace(
    time: Res<config::FrameTime>,
    mut selection: ResMut<game::Selection>,
    mut e_selection: EventWriter<game::ShowSelectionEvent>,
    mut q_selection: Query<(&mut game::Position,), With<game::SelectionMarker>>,
    mut displace: ResMut<ConveyorDisplace>,
    level: Res<level::Level>,
    display: Res<game::Display>,
) {
    if let Some(ref conveyor) = level.conveyor {
        let diff = time.diff() * conveyor.speed;
        displace.0 += diff;
        if displace.0 >= SLOT_SIZE.x {
            if let Some((pos, _)) = selection.iter().enumerate().find(|(_, id)| **id == 0) {
                selection.remove(pos);
                selection.push(0);
                e_selection.send(game::ShowSelectionEvent);
            }
            displace.0 = 0.0;
        } else {
            q_selection.par_iter_mut().for_each(|(mut pos,)| {
                if pos.x > sprite::SlotIndex(0).into_position(display.ratio).x {
                    pos.x -= diff;
                }
            });
        }
    }
}

fn highlighter_displace(
    mut q_highlighter: Query<(&mut game::Position,), With<game::SelectionHighlighter>>,
    selecting: Res<game::Selecting>,
    displace: Res<ConveyorDisplace>,
    level: Res<level::Level>,
    time: Res<config::FrameTime>,
) {
    if let Some(ref conveyor) = level.conveyor {
        if let Ok((mut pos,)) = q_highlighter.get_single_mut() {
            if selecting.is_changed() {
                pos.x -= displace.0;
            } else {
                pos.x -= time.diff() * conveyor.speed;
            }
        }
    }
}

fn conveyor_used(
    mut planter: EventReader<plants::PlanterEvent>,
    mut selection: ResMut<game::Selection>,
) {
    planter.read().for_each(|planter| {
        if let Some(id) = selection.get_mut(planter.index) {
            *id = 0;
        }
    });
}
