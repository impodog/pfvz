use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct PlantsFactorsPlugin;

impl Plugin for PlantsFactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peashooter {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub pea_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub interval: u64,
    pub cost: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct PlantFactors {
    pub peashooter: Peashooter,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/plants.toml").expect("cannot read plant factors");
    let factors: PlantFactors = toml::from_str(&str).expect("cannot parse plant factors");
    commands.insert_resource(factors);
}
