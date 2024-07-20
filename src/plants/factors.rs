use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct FactorsPlugin;

impl Plugin for FactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peashooter {
    pub velocity: game::Velocity,
    pub interval: u64,
    pub self_box: game::HitBox,
    pub pea_box: game::HitBox,
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
