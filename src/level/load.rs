use crate::prelude::*;

pub(super) struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>();
        app.add_systems(PreUpdate, (load_level,));
        // NOTE: This is for game testing
        #[cfg(debug_assertions)]
        app.add_systems(PostStartup, |mut e_level: EventWriter<LevelEvent>| {
            e_level.send(LevelEvent { stage: 1, level: 1 });
        });
    }
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
    mut selection: ResMut<game::Selection>,
) {
    if let Some(level) = e_level.read().last() {
        let path = format!("assets/levels/{}/{}.toml", level.stage, level.level);
        match std::fs::read_to_string(path) {
            Ok(content) => match toml::from_str::<level::Level>(&content) {
                Ok(level) => {
                    let size = level.config.layout.size();
                    let ratio = (LOGICAL_WIDTH / size.0 as f32).min(LOGICAL_HEIGHT / size.1 as f32)
                        * 2.0
                        / 3.0;
                    level.config.selection.modify(selection.as_mut());
                    commands.insert_resource(level);
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
