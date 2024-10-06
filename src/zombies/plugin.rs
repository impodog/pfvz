use super::*;
use crate::prelude::*;

pub struct ZombiesPlugin;

impl Plugin for ZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            factors::ZombiesFactorsPlugin,
            basic::ZombiesBasicPlugin,
            all_star::ZombiesAllStarPlugin,
            newspaper::ZombiesNewspaperPlugin,
            trashcan::ZombiesTrashcanPlugin,
            hidden::ZombiesHiddenPlugin,
            snorkel::ZombiesSnorkelPlugin,
            zomboni::ZombiesZomboniPlugin,
            dance::ZombiesDancePlugin,
            jitb::ZombiesJitbPlugin,
            balloon::ZombiesBalloonPlugin,
            digger::ZombiesDiggerPlugin,
            pogo::ZombiesPogoPlugin,
            gargantuar::ZombiesGargantuarPlugin,
            baseball::ZombiesBaseballPlugin,
        ));
        app.add_plugins((zomboss::ZombiesZombossPlugin,));
    }
}
