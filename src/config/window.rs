use crate::prelude::*;

pub(super) struct ConfigWindowPlugin;

impl Plugin for ConfigWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (modify_window,));
    }
}

fn modify_window(mut q_window: Query<&mut Window>) {
    let mut window = q_window.single_mut();
    window.resolution.set(LOGICAL_WIDTH, LOGICAL_HEIGHT);
    window.title = "Plants & Fungi v.s. Zombies".to_string();
}
