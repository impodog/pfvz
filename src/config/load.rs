use crate::prelude::*;
use serde::*;

pub(super) struct ConfigLoadPlugin;

impl Plugin for ConfigLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_config,));
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigProgramFramerate(pub f32);

impl Default for ConfigProgramFramerate {
    fn default() -> Self {
        Self(30.0)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigProgram {
    #[serde(default)]
    pub framerate: ConfigProgramFramerate,
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub program: ConfigProgram,
}

fn load_config(mut commands: Commands) {
    if let Ok(str) = std::fs::read_to_string("config.toml") {
        let config: Config = toml::from_str(&str).expect("cannot parse configuration");
        commands.insert_resource(config);
    } else {
        warn!("Unable to find \"config.toml\", using default");
        commands.init_resource::<Config>();
    }
}
