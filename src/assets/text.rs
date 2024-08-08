use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct AssetsTextPlugin;

impl Plugin for AssetsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_interface, init_creatures, init_dave));
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TextInterfaceWin {
    pub get_plant: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct TextInterface {
    pub win: TextInterfaceWin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextCreaturesDesc {
    pub name: String,
    pub short: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct TextCreatures {
    pub desc: HashMap<String, TextCreaturesDesc>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Deref, DerefMut)]
pub struct TextDavePart(pub Vec<String>);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TextDaveSerde {
    map: Vec<(level::LevelIndex, TextDavePart)>,
}
#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct TextDave {
    map: BTreeMap<level::LevelIndex, TextDavePart>,
}
impl From<TextDaveSerde> for TextDave {
    fn from(value: TextDaveSerde) -> Self {
        Self {
            map: BTreeMap::from_iter(value.map),
        }
    }
}

fn init_interface(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/text/interface.toml").expect("Cannot read interface text");
    match toml::from_str::<TextInterface>(&str) {
        Ok(text) => {
            commands.insert_resource(text);
        }
        Err(err) => {
            commands.insert_resource(TextInterface::default());
            error!("Unable to parse interface text: {}", err);
        }
    }
}

fn init_creatures(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/text/creatures.toml").expect("Cannot read creature text");
    match toml::from_str::<TextCreatures>(&str) {
        Ok(text) => {
            commands.insert_resource(text);
        }
        Err(err) => {
            commands.insert_resource(TextCreatures::default());
            error!("Unable to parse creature text: {}", err);
        }
    }
}

fn init_dave(mut commands: Commands) {
    let str = std::fs::read_to_string("assets/text/dave.toml").expect("Cannot read Dave text");
    match toml::from_str::<TextDaveSerde>(&str) {
        Ok(text) => {
            commands.insert_resource(TextDave::from(text));
        }
        Err(err) => {
            commands.insert_resource(TextDave::default());
            error!("Unable to parse Dave text: {}", err);
        }
    }
}
