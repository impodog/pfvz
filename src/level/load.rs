use crate::prelude::*;

pub(super) struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>();
        app.add_systems(PreUpdate, (load_level,));
        app.add_systems(OnEnter(info::GlobalStates::Play), (insert_zomboss_config,));
    }
}

#[derive(Event, Debug, Clone)]
pub struct LevelEvent {
    pub index: level::LevelIndex,
}

#[derive(Resource, Debug, Clone, Copy, Deref, DerefMut)]
pub struct LevelSlots(pub usize);

pub struct LevelRatioCalculator {
    pub x: usize,
    pub y: usize,
}
impl LevelRatioCalculator {
    fn into_f32(self) -> f32 {
        (LOGICAL_WIDTH / self.x as f32).min(LOGICAL_HEIGHT / self.y as f32) * 2.0 / 3.0
    }
}
lazy_static! {
    pub static ref FIXED_RATIO: f32 = LevelRatioCalculator { x: 9, y: 5 }.into_f32();
}

fn load_level(
    mut commands: Commands,
    mut e_level: EventReader<LevelEvent>,
    mut state: ResMut<NextState<info::GlobalStates>>,
    mut selection: ResMut<game::Selection>,
    map: Res<game::CreatureMap>,
    save: Res<save::Save>,
    items: Res<collectible::ItemFactors>,
) {
    if let Some(level_event) = e_level.read().last() {
        let path = format!(
            "assets/levels/{}/{}.toml",
            level_event.index.stage, level_event.index.level
        );
        match std::fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<level::Level>(&content) {
                Ok(level) => {
                    let size = level.config.layout.size();
                    commands.insert_resource(game::Display {
                        ratio: LevelRatioCalculator {
                            x: size.0,
                            y: size.1,
                        }
                        .into_f32(),
                    });

                    level.config.selection.modify(selection.as_mut());
                    let slots = if let level::SelectionArr::All(ref vec) = level.config.selection {
                        vec.len()
                    } else {
                        save.slots.max(level.config.selection.len())
                    };
                    commands.insert_resource(LevelSlots(slots));

                    let sun_factor = (level.config.sun as f32 / items.exciting.sun_standard as f32)
                        .powf(0.13)
                        .min(2.0);
                    let sum = level
                        .waves
                        .iter()
                        .fold((0.0, 0.0), |(acc, time), wave| {
                            let points = wave.points
                                + wave.fixed.iter().fold(0, |acc, (id, num)| {
                                    acc + map
                                        .get(id)
                                        .map(|creature| creature.cost)
                                        .unwrap_or_default()
                                        * (*num) as u32
                                });
                            let acc =
                                acc + points as f32 * (2.0 + 30.0 / (time + 10.0) - sun_factor);
                            (acc, time + wave.wait)
                        })
                        .0;
                    let factor = sum
                        / items.exciting.standard as f32
                        / (level.waves.len() as f32).powf(1.05);
                    let exciting = multiply_uf!(usize, items.exciting.zombies, factor.max(1.0));
                    let difficulty = level::RoomDifficulty {
                        sum,
                        exciting,
                        factor,
                    };
                    info!("Room difficulty: {:?}", difficulty);

                    commands.insert_resource(difficulty);

                    commands.insert_resource(level);
                    commands.insert_resource(level_event.index);
                    state.set(info::GlobalStates::Play);
                    info!("Loaded level and starting Play state");
                }
                Err(err) => {
                    error!("Failed to parse level: {}", err);
                }
            },
            Err(err) => {
                error!("Failed to open level: {}", err);
            }
        }
    }
}

pub fn insert_zomboss_config(mut commands: Commands, level: Res<level::Level>) {
    if let Some(ref zomboss) = level.zomboss {
        commands.insert_resource(zomboss.clone());
    }
}
