use crate::prelude::*;

pub(super) struct LevelRoomPlugin;

impl Plugin for LevelRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoomNextWave>();
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_room,));
        app.add_systems(
            Update,
            (update_room, spawn_zombies).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RoomStatus {
    pub cursor: usize,
    pub timer: Timer,
    pub min_timer: Timer,
    pub fin: bool,
}
impl Default for RoomStatus {
    fn default() -> Self {
        Self {
            cursor: 0,
            timer: Timer::default(),
            min_timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
            fin: false,
        }
    }
}

#[derive(Event)]
pub struct RoomNextWave(pub usize);

fn init_room(mut commands: Commands) {
    commands.insert_resource(RoomStatus::default());
}

fn update_room(
    mut status: ResMut<RoomStatus>,
    level: Res<level::Level>,
    mut next_wave: EventWriter<RoomNextWave>,
    time: Res<config::FrameTime>,
    q_zombie: Query<(), With<game::Zombie>>,
) {
    status.timer.tick(time.delta());
    status.min_timer.tick(time.delta());
    if status.cursor < level.waves.len()
        && status.min_timer.finished()
        && (status.timer.just_finished() || q_zombie.iter().next().is_none())
    {
        next_wave.send(RoomNextWave(status.cursor));
        info!("Updated to wave {}", status.cursor);
        status.timer = Timer::from_seconds(level.waves[status.cursor].wait, TimerMode::Once);
        status.min_timer.reset();
        status.cursor += 1;

        if status.cursor >= level.waves.len() {
            status.fin = true;
        }
    }
}

fn spawn_zombies(
    mut action: EventWriter<game::CreatureAction>,
    mut next_wave: EventReader<RoomNextWave>,
    level: Res<level::Level>,
    map: Res<game::CreatureMap>,
) {
    fn randomize(chances: &mut [usize], sum: &mut usize) -> usize {
        let mut len = rand::thread_rng().gen_range(0..*sum);
        for (i, item) in chances.iter_mut().enumerate() {
            len = len.saturating_sub(*item);
            if len == 0 {
                if *item > 1 {
                    *item -= 1;
                    *sum -= 1;
                }
                return i;
            }
        }
        unreachable!();
    }
    next_wave.read().for_each(|wave| {
        let wave = wave.0;
        let size = level.config.layout.size();
        let mut chances = vec![5usize; size.1];
        let mut sum = chances.iter().fold(0, |prev, cur| prev + *cur);
        let mut points = level.waves[wave].points;
        for (id, times) in &level.waves[wave].fixed {
            for _ in 0..*times {
                action.send(game::CreatureAction::Spawn(
                    *id,
                    game::Position::new_xy(
                        size.0 as f32 / 2.0,
                        (randomize(&mut chances, &mut sum) as f32 - size.1 as f32 / 2.0 + 0.5)
                            as i32 as f32,
                    )
                    .regularize(),
                ));
            }
        }
        while points > 0 {
            let i = rand::thread_rng().gen_range(0..level.waves[wave].avail.len());
            let id = level.waves[wave].avail[i];
            if let Some(creature) = map.get(&id) {
                points = points.saturating_sub(creature.cost);
            } else {
                warn!("Attempt to spawn non-existing creature {}", i);
            }
            action.send(game::CreatureAction::Spawn(
                id,
                game::Position::new_xy(
                    size.0 as f32 / 2.0,
                    (randomize(&mut chances, &mut sum) as f32 - size.1 as f32 / 2.0 + 0.5) as i32
                        as f32,
                )
                .regularize(),
            ));
        }
    });
}
