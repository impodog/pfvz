use crate::prelude::*;

pub(super) struct MenuMainPlugin;

impl Plugin for MenuMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (main_menu,).run_if(when_state!(main)));
    }
}

fn main_menu(mut contexts: EguiContexts, mut menu: ResMut<NextState<info::MenuStates>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.label(egui::RichText::new("Plants & Fungi v.s. Zombies").size(50.0 * UI_ZOOM_FACTOR));
        if ui.button(super::medium_text("Adventure")).clicked() {
            menu.set(info::MenuStates::Adventure);
        }
    });
}
