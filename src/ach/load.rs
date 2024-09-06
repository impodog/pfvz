use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct AchLoadPlugin;

impl Plugin for AchLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_achievements,));
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchVisibility {
    #[default]
    Full,
    NameOnly,
    Hidden,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ach {
    pub name: String,
    pub desc: String,
    #[serde(default)]
    pub vis: AchVisibility,
}

#[derive(Serialize, Deserialize)]
struct AchievementsSerde(BTreeMap<String, Ach>);

#[derive(Resource, Deref, DerefMut)]
pub struct Achievements(pub BTreeMap<ach::AchId, Ach>);

impl TryFrom<AchievementsSerde> for Achievements {
    type Error = Vec<ach::AchId>;

    fn try_from(mut value: AchievementsSerde) -> Result<Achievements, Self::Error> {
        let mut unused = Vec::new();
        let values = enum_iterator::all::<ach::AchId>()
            .filter_map(|ach| match value.0.remove(ach.name()) {
                Some(value) => Some((ach, value)),
                None => {
                    unused.push(ach);
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();
        if unused.is_empty() {
            Ok(Achievements(values))
        } else {
            Err(unused)
        }
    }
}

fn load_achievements(mut commands: Commands) {
    let str = std::fs::read_to_string("assets/text/achievements.toml")
        .expect("cannot open achievements file");
    match toml::from_str::<AchievementsSerde>(&str) {
        Ok(ach) => match Achievements::try_from(ach) {
            Ok(ach) => {
                commands.insert_resource(ach);
            }
            Err(err) => {
                error!(
                    "When loading achievements, these text cannot be found: {:?}",
                    err
                );
            }
        },
        Err(err) => {
            error!("Unable to parse achievements file: {}", err);
        }
    }
}
