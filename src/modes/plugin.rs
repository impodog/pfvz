use super::*;
use crate::prelude::*;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            whack::ModesWhackPlugin,
            quick_shot::ModesQuickShotPlugin,
            fog::ModesFogPlugin,
            random::ModesRandomPlugin,
            thunder::ModesThunderPlugin,
            intro::ModesIntroPlugin,
            roof::ModesRoofPlugin,
            columns::ModesColumnsPlugin,
            infi_sun::ModesInfiSunPlugin,
        ));
    }
}
