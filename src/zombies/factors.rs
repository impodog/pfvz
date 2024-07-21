use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct FactorsPlugin;

impl Plugin for FactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicZombie {
    pub velocity: game::Velocity,
    pub self_box: game::HitBox,
    pub arm_box: game::HitBox,
    pub self_health: (u32, u32),
    pub arm_health: u32,
    pub damage: u32,
    pub interval: u64,
    pub cost: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct ZombieFactors {
    pub basic: BasicZombie,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/zombies.toml").expect("cannot read zombie factors");
    let factors: ZombieFactors = toml::from_str(&str).expect("cannot parse zombie factors");
    commands.insert_resource(factors);
}
