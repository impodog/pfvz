use crate::prelude::*;

pub(super) struct ConfigWindowPlugin;

impl Plugin for ConfigWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (modify_window,));
    }
}

fn modify_window(mut q_window: Query<&mut Window>) {
    let mut window = q_window.single_mut();
    window.resolution.set_scale_factor_override(Some(1.0));
    window.title = "Plants & Fungi v.s. Zombies".to_string();
}
