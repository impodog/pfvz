use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct AssetsTextPlugin;

impl Plugin for AssetsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_interface, init_creatures));
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TextInterfaceWin {
    pub get_plant: String,
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
