use crate::prelude::*;

pub(super) struct ModesInfiSunPlugin;

impl Plugin for ModesInfiSunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (set_to_inf,).run_if(when_state!(gaming)));
    }
}

fn set_to_inf(level: Res<level::Level>, mut sun: ResMut<game::Sun>) {
    if sun.0 != SUN_MAGIC && level.config.game.contains(&level::GameKind::InfiSun) {
        sun.0 = SUN_MAGIC;
    }
}
