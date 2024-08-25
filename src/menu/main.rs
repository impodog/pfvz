use crate::prelude::*;

pub(super) struct MenuMainPlugin;

impl Plugin for MenuMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (main_menu,).run_if(when_state!(main)));
    }
}

fn main_menu(mut contexts: EguiContexts, mut menu: ResMut<NextState<info::MenuStates>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label(super::title_text("Plants & Fungi vs. Zombies"));
        if ui.button(super::medium_text("Adventure")).clicked() {
            menu.set(info::MenuStates::Adventure);
        }
        if ui.button(super::medium_text("Config")).clicked() {
            menu.set(info::MenuStates::Config);
        }
    });
}
