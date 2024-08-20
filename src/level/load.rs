use crate::prelude::*;

pub(super) struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>();
        app.add_systems(PreUpdate, (load_level,));
    }
}

#[derive(Event, Debug, Clone)]
pub struct LevelEvent {
    pub index: level::LevelIndex,
}

#[derive(Resource, Debug, Clone, Copy, Deref, DerefMut)]
pub struct LevelSlots(pub usize);

fn load_level(
    mut commands: Commands,
    mut e_level: EventReader<LevelEvent>,
    mut state: ResMut<NextState<info::GlobalStates>>,
    mut selection: ResMut<game::Selection>,
    save: Res<save::Save>,
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
                    let ratio = (LOGICAL_WIDTH / size.0 as f32).min(LOGICAL_HEIGHT / size.1 as f32)
                        * 2.0
                        / 3.0;
                    level.config.selection.modify(selection.as_mut());
                    let slots = if let level::SelectionArr::All(ref vec) = level.config.selection {
                        vec.len()
                    } else {
                        save.slots.max(level.config.selection.len())
                    };
                    commands.insert_resource(LevelSlots(slots));
                    commands.insert_resource(level);
                    commands.insert_resource(level_event.index);
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
