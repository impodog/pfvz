use crate::prelude::*;

pub(super) struct MenuConfigPlugin;

impl Plugin for MenuConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (config_menu,).run_if(when_state!(config)));
    }
}

#[derive(Resource, Debug)]
struct ConfigData {
    sun_value: String,
    speed: String,
    damage: String,
}
impl From<&config::Config> for ConfigData {
    fn from(value: &config::Config) -> Self {
        Self {
            sun_value: value.gamerule.sun_value.0.to_string(),
            speed: value.gamerule.speed.0.to_string(),
            damage: value.gamerule.damage.0.to_string(),
        }
    }
}
impl ConfigData {
    pub fn modify(&self, config: &mut config::Config) -> anyhow::Result<()> {
        config.gamerule.sun_value.0 = self.sun_value.parse()?;
        config.gamerule.speed.0 = self.speed.parse()?;
        config.gamerule.damage.0 = self.damage.parse()?;
        Ok(())
    }
}

fn init_config(mut commands: Commands, config: Res<config::Config>) {
    commands.insert_resource(ConfigData::from(&*config));
}

fn config_menu(
    mut contexts: EguiContexts,
    mut menu: ResMut<NextState<info::MenuStates>>,
    mut exit: EventWriter<AppExit>,
    mut config: ResMut<config::Config>,
    mut data: ResMut<ConfigData>,
    mut error: Local<String>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label(super::title_text("Configurations"));

        ui.label(super::large_text("Game Rule"));

        ui.label(super::small_text("Sun value"));
        ui.add(super::medium_edit(&mut data.sun_value));

        ui.label(super::small_text("Game Speed"));
        ui.add(super::medium_edit(&mut data.speed));

        ui.label(super::small_text("Plant Damage Multiplier"));
        ui.add(super::medium_edit(&mut data.damage));

        ui.label(super::large_text("Program-specific"));

        ui.label(super::small_text(
            "(You need to restart the program to apply configurations.)",
        ));

        if ui
            .button(super::medium_text("Apply and Exit Program"))
            .clicked()
        {
            match data.modify(&mut config) {
                Ok(()) => {
                    exit.send(AppExit::Success);
                }
                Err(err) => {
                    *error = err.to_string();
                }
            }
        }
        if ui.button(super::medium_text("Back to Main Menu")).clicked() {
            menu.set(info::MenuStates::Main);
        }
        if !error.is_empty() {
            ui.label(super::medium_text(error.as_str()));
        }
    });
}
