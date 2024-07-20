use super::*;
use crate::prelude::*;

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((peas::PlantsPeaPlugin, factors::FactorsPlugin));
    }
}
