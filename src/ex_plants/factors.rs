use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ExPlantsFactorsPlugin;

impl Plugin for ExPlantsFactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwinSunflower {
    pub velocity: game::VelocityAny,
    pub self_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
    pub multiplier: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HomingThistle {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub prick_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct ExPlantFactors {
    pub twin_sunflower: TwinSunflower,
    pub homing_thistle: HomingThistle,
}

fn init_factors(mut commands: Commands) {
    let str = std::fs::read_to_string("assets/factors/ex_plants.toml")
        .expect("cannot read ex-plant factors");
    let factors: ExPlantFactors = toml::from_str(&str).expect("cannot parse ex-plant factors");
    commands.insert_resource(factors);
}
