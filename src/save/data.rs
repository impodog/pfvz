use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct SaveDataPlugin;

impl Plugin for SaveDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_save,));
        app.add_systems(Last, (save_save,));
    }
}

configuration!(SaveSlots, usize, 6);
configuration!(SaveMoney, i32, 0);
configuration!(
    SaveAdventure,
    level::LevelIndex,
    level::LevelIndex { stage: 1, level: 1 }
);
#[derive(Serialize, Deserialize, Resource, Debug, Default, Clone)]
pub struct Save {
    pub slots: SaveSlots,
    pub selection: game::Selection,
    pub money: SaveMoney,
    pub plants: BTreeSet<Id>,
    pub adventure: SaveAdventure,
}

fn load_save(mut commands: Commands) {
    if let Some(save) = std::fs::read_to_string("save.json")
        .ok()
        .and_then(|s| serde_json::from_str::<Save>(&s).ok())
    {
        commands.insert_resource(save);
    } else {
        warn!("No save file available, using default");
        commands.init_resource::<Save>();
    }
}

fn save_save(mut e_exit: EventReader<AppExit>, save: Res<Save>) {
    e_exit.read().for_each(|_| {
        let s = serde_json::to_string(save.as_ref()).unwrap();
        std::fs::write("save.json", s).expect("Failed to write save file");
    });
}
