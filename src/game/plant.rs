use crate::prelude::*;

pub(super) struct GamePlantPlugin;

impl Plugin for GamePlantPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Plant;

#[derive(Component)]
pub struct PlantRelevant;
