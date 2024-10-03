use crate::prelude::*;

pub(super) struct MenuMainPlugin;

impl Plugin for MenuMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (setup,));
        app.add_systems(Update, (main_menu,).run_if(when_state!(main)));
    }
}

fn setup(mut contexts: EguiContexts) {
    contexts.ctx_mut().style_mut(|style| {
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(30.0, egui::FontFamily::Monospace),
        );
    });
}

fn main_menu(mut contexts: EguiContexts, mut menu: ResMut<NextState<info::MenuStates>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label(super::title_text("Plants & Fungi vs. Zombies"));
        if ui.button(super::medium_text("Adventure")).clicked() {
            menu.set(info::MenuStates::Adventure);
        }
        if ui.button(super::medium_text("Achievements")).clicked() {
            menu.set(info::MenuStates::Achievements);
        }
        if ui.button(super::medium_text("Almanac")).clicked() {
            menu.set(info::MenuStates::Almanac);
        }
        if ui.button(super::medium_text("Config")).clicked() {
            menu.set(info::MenuStates::Config);
        }
        if ui.button(super::medium_text("Credits & License")).clicked() {
            menu.set(info::MenuStates::Credits);
        }
    });
}
