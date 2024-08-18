use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ZombiesFactorsPlugin;

impl Plugin for ZombiesFactorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_factors,));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicZombie {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub arm_box: game::HitBox,
    pub self_health: (u32, u32),
    pub arm_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoadconeZombie {
    pub roadcone_box: game::HitBox,
    pub roadcone_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BucketZombie {
    pub bucket_box: game::HitBox,
    pub bucket_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlagZombie {
    pub velocity: game::VelocityXRange,
    pub flag_box: game::HitBox,
    pub flag_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllStarZombie {
    pub velocity_running: game::VelocityXRange,
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub helmet_box: game::HitBox,
    pub self_health: (u32, u32),
    pub helmet_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub tackle_damage: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewspaperZombie {
    pub velocity: game::VelocityXRange,
    pub rage_velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub newspaper_box: game::HitBox,
    pub self_health: (u32, u32),
    pub newspaper_health: u32,
    pub damage: u32,
    pub interval: f32,
    pub rage_interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScreenDoorZombie {
    pub screen_door_box: game::HitBox,
    pub screen_door_health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrashcanZombie {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub trashcan_box: game::HitBox,
    pub self_health: (u32, u32),
    pub trashcan_health: (u32, u32),
    pub damage: u32,
    pub trashcan_damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HiddenZombie {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub self_health: (u32, u32),
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tube {
    pub self_box: game::HitBox,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnorkelZombie {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub underwater_box: game::HitBox,
    pub self_health: (u32, u32),
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zomboni {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub self_health: (u32, u32),
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DancingZombie {
    pub velocity_back: game::VelocityXRange,
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub backup_box: game::HitBox,
    pub self_health: (u32, u32),
    pub backup_health: (u32, u32),
    pub damage: u32,
    pub interval: f32,
    pub back_time: f32,
    pub spawn_interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JitbZombie {
    pub velocity: game::VelocityXRange,
    pub self_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub self_health: (u32, u32),
    pub damage: u32,
    pub animation_time: f32,
    pub cooldown: f32,
    pub cost: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct ZombieFactors {
    pub basic: BasicZombie,
    pub roadcone: RoadconeZombie,
    pub bucket: BucketZombie,
    pub flag: FlagZombie,
    pub all_star: AllStarZombie,
    pub newspaper: NewspaperZombie,
    pub screen_door: ScreenDoorZombie,
    pub trashcan: TrashcanZombie,
    pub hidden: HiddenZombie,
    pub tube: Tube,
    pub snorkel: SnorkelZombie,
    pub zomboni: Zomboni,
    pub dancing: DancingZombie,
    pub jitb: JitbZombie,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/zombies.toml").expect("cannot read zombie factors");
    let factors: ZombieFactors = toml::from_str(&str).expect("cannot parse zombie factors");
    commands.insert_resource(factors);
}
