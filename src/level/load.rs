use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>();
        app.add_systems(PreUpdate, (load_level,));
        #[cfg(debug_assertions)]
        app.add_systems(PostStartup, |mut e_level: EventWriter<LevelEvent>| {
            e_level.send(LevelEvent { stage: 1, level: 1 });
        });
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Wave {
    pub points: u32,
    pub big: bool,
    pub fixed: Vec<(Id, usize)>,
    pub avail: Vec<(Id, usize)>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum LayoutKind {
    #[default]
    Day,
}
impl LayoutKind {
    pub fn size(&self) -> (usize, usize) {
        match self {
            Self::Day => (9, 5),
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum GameKind {
    #[default]
    Adventure,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct LevelConfig {
    pub layout: LayoutKind,
    pub game: GameKind,
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct Level {
    pub waves: Vec<Wave>,
    pub config: LevelConfig,
}

#[derive(Event, Debug)]
pub struct LevelEvent {
    pub stage: u8,
    pub level: u8,
}

fn load_level(
    mut commands: Commands,
    mut e_level: EventReader<LevelEvent>,
    mut state: ResMut<NextState<info::GlobalStates>>,
) {
    if let Some(level) = e_level.read().last() {
        let path = format!("assets/levels/{}/{}.toml", level.stage, level.level);
        match std::fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<Level>(&content) {
                Ok(level) => {
                    let size = level.config.layout.size();
                    let ratio = (LOGICAL_WIDTH / size.0 as f32).min(LOGICAL_HEIGHT / size.1 as f32)
                        * 2.0
                        / 3.0;
                    commands.insert_resource(level);
                    commands.insert_resource(level::Room::default());
                    commands.insert_resource(game::Display { ratio });
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
