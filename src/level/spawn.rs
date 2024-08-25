use crate::prelude::*;

pub(super) struct LevelSpawnPlugin;

impl Plugin for LevelSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (init_level,));
        app.add_systems(
            Update,
            (increase_points, by_probability).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct LevelPoints(pub u32);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct SpawnProbability(pub Vec<u32>);

#[derive(Resource, Debug, Default, Clone, Deref, DerefMut)]
pub struct SpawnStack(pub Vec<Id>);

#[derive(Resource, Debug, Default, Clone, Deref, DerefMut)]
pub struct SpawnGuard(pub bool);

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
struct CurrentWave(pub usize);

fn init_level(mut commands: Commands, level: Res<level::Level>) {
    commands.insert_resource(LevelPoints::default());
    commands.insert_resource(SpawnProbability(vec![
        SPARSENESS;
        level.config.layout.size().1
    ]));
    commands.insert_resource(CurrentWave::default());
    commands.insert_resource(SpawnStack::default());
    commands.insert_resource(SpawnGuard::default());
}

fn increase_points(
    mut next_wave: EventReader<level::RoomNextWave>,
    mut points: ResMut<LevelPoints>,
    level: Res<level::Level>,
    mut stack: ResMut<SpawnStack>,
    mut guard: ResMut<SpawnGuard>,
    mut current: ResMut<CurrentWave>,
) {
    next_wave.read().for_each(|wave| {
        if let Some(wave) = level.waves.get(wave.0) {
            **points += wave.points;
            for fixed in wave.fixed.iter() {
                for _ in 0..fixed.1 {
                    stack.push(fixed.0);
                }
            }
            // stack.shuffle(&mut rand::thread_rng());
            **guard = true;
        }
        **current = wave.0;
    });
}

#[allow(clippy::too_many_arguments)]
fn by_probability(
    mut action: EventWriter<game::CreatureAction>,
    mut points: ResMut<LevelPoints>,
    mut prob: ResMut<SpawnProbability>,
    mut stack: ResMut<SpawnStack>,
    mut guard: ResMut<SpawnGuard>,
    map: Res<game::CreatureMap>,
    current: Res<CurrentWave>,
    level: Res<level::Level>,
    q_banner: Query<(), With<level::Banner>>,
) {
    // guard condition
    if !guard.0 {
        return;
    }
    // Wait until the banner disappears
    if q_banner.iter().next().is_some() {
        return;
    }

    // This will be modified to true again if spawning is completed
    guard.0 = false;

    let (id, cost) = if stack.is_empty() {
        (
            level.waves.get(current.0).and_then(|wave| {
                let weight = wave
                    .avail
                    .iter()
                    .map(|id| {
                        (
                            *id,
                            map.get(id).map(creature_popularity).unwrap_or_default(),
                        )
                    })
                    .collect::<Vec<_>>();
                weight
                    .choose_weighted(&mut rand::thread_rng(), |item| item.1)
                    .ok()
                    .map(|item| item.0)
            }),
            true,
        )
    } else {
        (stack.pop(), false)
    };
    if let Some(id) = id {
        let creature = if let Some(creature) = map.get(&id) {
            creature
        } else {
            error!("Unknown creature id {} in level file", id);
            return;
        };

        let mut index: usize;
        let mut count = 0usize;
        // Only 10 attempts allowed
        // When level creator mistakenly puts incompatible zombies, count will exceed this limit
        loop {
            // Multiply all probability(with caps)
            for value in prob.iter_mut() {
                if *value < SPARSENESS {
                    *value = (*value * 3 / 2).min(SPARSENESS);
                }
            }
            // Randomly select a index accordingly
            index = {
                let dist = WeightedIndex::new(&prob.0).unwrap();
                dist.sample(&mut rand::thread_rng())
            };

            // Only spawn compatible creatures
            if level.config.layout.get_lane(index).is_compat(creature) {
                break;
            }
            count += 1;
            if count >= 15 {
                break;
            }
        }

        let hsize = level.config.layout.half_size_f32();

        // This mostly prevents overlapping zombies
        let get_x = || hsize.0 + 0.5 + rand::thread_rng().gen_range(-0.2..=0.2);
        let y = index as f32 - hsize.1;
        if let Some(creature) = map.get(&id) {
            // Define if a new zombie can be spawn
            let ok = if cost && points.0 >= creature.cost {
                points.0 -= creature.cost;
                true
            } else {
                !cost
            };
            prob.0[index] = (prob.0[index].saturating_sub(creature.cost * 2)).max(1);

            if ok {
                let x = get_x();
                let mut pos = level
                    .config
                    .layout
                    .regularize_xyzr(&game::Position::new_xy(x, y));
                pos.x = x;
                action.send(game::CreatureAction::Spawn(
                    id,
                    game::LogicPosition::from_base(pos),
                ));
                // Continue on to spawning
                guard.0 = true;
            }
        }
    }
}
