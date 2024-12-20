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
    pub times: usize,
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
pub struct SnowPea {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub pea_box: game::HitBox,
    pub snow: compn::SnowSerde,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repeater {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IcebergLettuce {
    pub self_box: game::HitBox,
    pub snow: compn::SnowSerde,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PuffShroom {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub spore_box: game::HitBox,
    pub range: game::PositionRangeX,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SunShroom {
    pub velocity: game::VelocityAny,
    pub small_box: game::HitBox,
    pub big_box: game::HitBox,
    pub health: u32,
    pub interval: f32,
    pub grow_interval: f32,
    pub cooldown: f32,
    pub cost: u32,
    pub small_multiplier: f32,
    pub big_multiplier: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraveBuster {
    pub self_box: game::HitBox,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FumeShroom {
    pub self_box: game::HitBox,
    pub fume_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScaredyShroom {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub scare_range: game::PositionRange,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IceShroom {
    pub self_box: game::HitBox,
    pub snow: compn::SnowSerde,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoomShroom {
    pub self_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SunBean {
    pub self_box: game::HitBox,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub max: u32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LilyPad {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Squash {
    pub self_box: game::HitBox,
    pub range: game::PositionRangeXStartEnd,
    pub jump_height: f32,
    pub time: f32,
    pub health: u32,
    pub damage: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Threepeater {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TallNut {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spikeweed {
    pub self_box: game::HitBox,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Torchwood {
    pub self_box: game::HitBox,
    pub light_range: game::PositionRangeSerde,
    pub fire: compn::FireProjectileSerde,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BonkChoy {
    pub self_box: game::HitBox,
    pub range: game::PositionRangeXStartEnd,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jalapeno {
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
pub struct Plantern {
    pub self_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hurrikale {
    pub self_box: game::HitBox,
    pub blow_velocity: f32,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlowerPot {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pumpkin {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Starfruit {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub star_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MagnetShroom {
    pub self_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub objects: usize,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blover {
    pub self_box: game::HitBox,
    pub health: u32,
    pub velocity_factor: f32,
    pub duration: f32,
    pub fog_duration: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cactus {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub spike_box: game::HitBox,
    pub times: usize,
    pub pierce: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CabbagePult {
    pub velocity: game::VelocityLobber,
    pub self_box: game::HitBox,
    pub cabbage_box: game::HitBox,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoffeeBean {
    pub self_box: game::HitBox,
    pub health: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KernelPult {
    pub velocity: game::VelocityLobber,
    pub self_box: game::HitBox,
    pub kernel_box: game::HitBox,
    pub butter_box: game::HitBox,
    pub snow: compn::SnowSerde,
    pub butter_every: usize,
    pub times: usize,
    pub health: u32,
    pub kernel_damage: u32,
    pub butter_damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Garlic {
    pub velocity: game::VelocityX,
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MelonPult {
    pub velocity: game::VelocityLobber,
    pub self_box: game::HitBox,
    pub melon_box: game::HitBox,
    pub fire: compn::FireProjectileSerde,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ethylene {
    pub self_box: game::HitBox,
    pub health: u32,
    pub duration: f32,
    pub factor: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SapFling {
    pub velocity: game::VelocityLobber,
    pub self_box: game::HitBox,
    pub pine_box: game::HitBox,
    pub range: game::PositionRangeSerde,
    pub snow: compn::SnowSerde,
    pub times: usize,
    pub health: u32,
    pub damage: u32,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoldBloom {
    pub velocity: game::VelocityAny,
    pub self_box: game::HitBox,
    pub health: u32,
    pub times: usize,
    pub night_times: usize,
    pub interval: f32,
    pub cooldown: f32,
    pub cost: u32,
    pub multiplier: f32,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grave {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crater {
    pub self_box: game::HitBox,
    pub health: u32,
    pub cooldown: f32,
    pub cost: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ice {
    pub self_box: game::HitBox,
    pub health: u32,
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
    pub snow_pea: SnowPea,
    pub repeater: Repeater,
    pub iceberg_lettuce: IcebergLettuce,
    pub puff_shroom: PuffShroom,
    pub sun_shroom: SunShroom,
    pub grave_buster: GraveBuster,
    pub fume_shroom: FumeShroom,
    pub scaredy_shroom: ScaredyShroom,
    pub ice_shroom: IceShroom,
    pub doom_shroom: DoomShroom,
    pub sun_bean: SunBean,
    pub lily_pad: LilyPad,
    pub squash: Squash,
    pub threepeater: Threepeater,
    pub tall_nut: TallNut,
    pub spikeweed: Spikeweed,
    pub torchwood: Torchwood,
    pub bonk_choy: BonkChoy,
    pub jalapeno: Jalapeno,
    pub plantern: Plantern,
    pub hurrikale: Hurrikale,
    pub flower_pot: FlowerPot,
    pub pumpkin: Pumpkin,
    pub starfruit: Starfruit,
    pub magnet_shroom: MagnetShroom,
    pub blover: Blover,
    pub cactus: Cactus,
    pub cabbage_pult: CabbagePult,
    pub coffee_bean: CoffeeBean,
    pub kernel_pult: KernelPult,
    pub garlic: Garlic,
    pub melon_pult: MelonPult,
    pub ethylene: Ethylene,
    pub sap_fling: SapFling,
    pub gold_bloom: GoldBloom,
    pub bowling_nut: BowlingNut,
    pub grave: Grave,
    pub crater: Crater,
    pub ice: Ice,
}

fn init_factors(mut commands: Commands) {
    let str =
        std::fs::read_to_string("assets/factors/plants.toml").expect("cannot read plant factors");
    let factors: PlantFactors = toml::from_str(&str).expect("cannot parse plant factors");
    commands.insert_resource(factors);
}
