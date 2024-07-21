use super::*;
use crate::prelude::*;

pub struct ZombiesPlugin;

impl Plugin for ZombiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((factors::FactorsPlugin, basic::ZombiesBasicPlugin));
    }
}
