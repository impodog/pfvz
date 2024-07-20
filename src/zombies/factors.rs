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
    velocity: game::Velocity,
    self_box: game::HitBox,
    arm_box: game::HitBox,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct ZombieFactors {
    basic: BasicZombie,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/zombies.toml").expect("cannot read zombie factors");
    let factors: ZombieFactors = toml::from_str(&str).expect("cannot parse zombie factors");
    commands.insert_resource(factors);
}
