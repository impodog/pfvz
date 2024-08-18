use crate::prelude::*;

pub(super) struct MenuAdventurePlugin;

impl Plugin for MenuAdventurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (adventure_menu,).run_if(when_state!(adventure)));
    }
}

fn adventure_menu(
    mut contexts: EguiContexts,
    mut e_level: EventWriter<level::LevelEvent>,
    mut stage: Local<String>,
    mut level: Local<String>,
    mut warning: Local<String>,
    mut trigger: Local<bool>,
    save: Res<save::Save>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label(super::medium_text("Choose Adventure Menu"));
        ui.label(super::medium_text(format!(
            "You unlocked {}",
            save.adventure.0
        )));
        ui.label(super::small_text("Stage"));
        ui.text_edit_singleline(&mut *stage);
        ui.label(super::small_text("Level"));
        ui.text_edit_singleline(&mut *level);
        if ui.button(super::medium_text("Start")).clicked() {
            let index = stage.parse().and_then(|stage| {
                level
                    .parse()
                    .map(|level| level::LevelIndex { stage, level })
            });
            match index {
                Ok(index) => {
                    if save.adventure.0 >= index {
                        e_level.send(level::LevelEvent { index });
                        *trigger = true;
                    } else {
                        *warning = "You haven't unlock this level yet!".into();
                    }
                }
                Err(err) => {
                    *warning = format!("{}", err);
                }
            }
        }
        if *trigger {
            *warning = "No such level!".into();
            *trigger = false;
        }
        if !warning.is_empty() {
            ui.label(super::medium_text(warning.clone()));
        }
    });
}