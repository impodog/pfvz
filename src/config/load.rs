use crate::prelude::*;
use serde::*;

pub(super) struct ConfigLoadPlugin;

impl Plugin for ConfigLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_config,));
        app.add_systems(Last, (save_config,));
    }
}

#[macro_export]
macro_rules! configuration {
    ($name: ident, $type: ty, $value: expr) => {
        #[derive(Serialize, Deserialize, Debug, Clone, Deref, DerefMut)]
        pub struct $name(pub $type);
        impl Default for $name {
            fn default() -> Self {
                Self($value)
            }
        }
    };
}

configuration!(ConfigProgramFramerate, f32, 40.0);
configuration!(ConfigProgramLossRate, (u32, u32), (5, 6));
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigProgram {
    pub framerate: ConfigProgramFramerate,
    pub loss_rate: ConfigProgramLossRate,
}

configuration!(ConfigGameRuleSunValue, u32, 25);
configuration!(ConfigGameRuleDamage, f32, 1.0);
configuration!(ConfigGameRuleSpeed, f32, 1.0);
configuration!(ConfigGameRuleGravity, f32, 2.8);
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigGameRule {
    pub sun_value: ConfigGameRuleSunValue,
    pub damage: ConfigGameRuleDamage,
    pub speed: ConfigGameRuleSpeed,
    pub gravity: ConfigGameRuleGravity,
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Config {
    pub program: ConfigProgram,
    pub gamerule: ConfigGameRule,
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

fn save_config(mut exit: EventReader<AppExit>, config: Res<Config>) {
    if exit.read().next().is_some() {
        let str = toml::to_string(&*config).expect("cannot serialize configuration");
        std::fs::write("config.toml", str).expect("cannot write configuration");
    }
}
