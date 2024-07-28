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
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sunflower {
    pub velocity: game::VelocityAny,
    pub self_box: game::HitBox,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
    pub multiplier: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CherryBomb {
    pub self_box: game::HitBox,
    pub boom_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub cooldown: f32,
    pub countdown: f32,
    pub animation_time: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WallNut {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PotatoMine {
    pub self_box: game::HitBox,
    pub boom_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub prepare: f32,
    pub cooldown: f32,
    pub animation_time: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BowlingNut {
    pub velocity: game::Velocity,
    pub self_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct PlantFactors {
    pub peashooter: Peashooter,
    pub sunflower: Sunflower,
    pub cherry_bomb: CherryBomb,
    pub wall_nut: WallNut,
    pub potato_mine: PotatoMine,
    pub bowling_nut: BowlingNut,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/plants.toml").expect("cannot read plant factors");
    let factors: PlantFactors = toml::from_str(&str).expect("cannot parse plant factors");
    commands.insert_resource(factors);
}
