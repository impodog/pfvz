use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct CollectibleFactorsPlugin;

impl Plugin for CollectibleFactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sun {
    pub velocity: game::Velocity,
    pub self_box: game::HitBox,
    pub height: f32,
    pub interval: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Whack {
    pub self_box: game::HitBox,
    pub damage: u32,
    pub interval: game::DurationRangeSecs,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fog {
    pub start: usize,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct ItemFactors {
    pub sun: Sun,
    pub whack: Whack,
    pub fog: Fog,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/items.toml").expect("cannot read item factors");
    let factors: ItemFactors = toml::from_str(&str).expect("cannot parse item factors");
    commands.insert_resource(factors);
}
