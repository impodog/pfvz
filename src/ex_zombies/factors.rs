use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ExZombiesFactorsPlugin;

impl Plugin for ExZombiesFactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RallyZombie {
    pub velocity: game::VelocityXRange,
    pub rally_flag_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub boost: f32,
    pub rally_flag_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrickZombie {
    pub brick_box: game::HitBox,
    pub brick_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GigaAllStarZombie {
    pub velocity_running: game::VelocityXRange,
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub helmet_box: game::HitBox,
    pub slide_distance: f32,
    pub self_health: (u32, u32),
    pub helmet_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub tackle_damage: u32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SundayEditionZombie {
    pub velocity: game::VelocityXRange,
    pub rage_velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub sunedit_box: game::HitBox,
    pub self_health: (u32, u32),
    pub sunedit_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub rage_interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorZombie {
    pub mirror_box: game::HitBox,
    pub mirror_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct ExZombieFactors {
    pub rally: RallyZombie,
    pub brick: BrickZombie,
    pub giga: GigaAllStarZombie,
    pub sunedit: SundayEditionZombie,
    pub mirror: MirrorZombie,
}

fn init_factors(mut commands: Commands) {
    let str = std::fs::read_to_string("assets/factors/ex_zombies.toml")
        .expect("cannot read ex-zombie factors");
    let factors: ExZombieFactors = toml::from_str(&str).expect("cannot parse ex-zombie factors");
    commands.insert_resource(factors);
}
