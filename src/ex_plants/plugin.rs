use super::*;
use crate::prelude::*;

pub struct ExPlantsPlugin;

impl Plugin for ExPlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExPlantsFactorsPlugin, ExPlantsProducerPlugin));
    }
}
