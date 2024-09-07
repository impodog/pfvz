use crate::prelude::*;

// Usage range for identifiers:
// + positives for zombies
// - negatives for plants
// (+/-) 1..100 => adventure creatures
// (+/-) 101..200 => mini game creatures
// (+/-) 201..300 => adventure non-player non-individual creatures

pub const PEASHOOTER: Id = -1;
pub const SUNFLOWER: Id = -2;
pub const CHERRY_BOMB: Id = -3;
pub const WALL_NUT: Id = -4;
pub const POTATO_MINE: Id = -5;
pub const SNOW_PEA: Id = -6;
pub const REPEATER: Id = -7;
pub const ICEBERG_LETTUCE: Id = -8;
pub const PUFF_SHROOM: Id = -9;
pub const SUN_SHROOM: Id = -10;
pub const GRAVE_BUSTER: Id = -11;
pub const FUME_SHROOM: Id = -12;
pub const SCAREDY_SHROOM: Id = -13;
pub const ICE_SHROOM: Id = -14;
pub const DOOM_SHROOM: Id = -15;
pub const SUN_BEAN: Id = -16;
pub const LILY_PAD: Id = -17;
pub const SQUASH: Id = -18;
pub const THREEPEATER: Id = -19;
pub const TALL_NUT: Id = -20;
pub const SPIKEWEED: Id = -21;
pub const TORCHWOOD: Id = -22;
pub const BONK_CHOY: Id = -23;
pub const JALAPENO: Id = -24;
pub const PLANTERN: Id = -25;
pub const HURRIKALE: Id = -26;
pub const FLOWER_POT: Id = -27;
pub const PUMPKIN: Id = -28;
pub const STARFRUIT: Id = -29;
pub const MAGNET_SHROOM: Id = -30;
pub const BLOVER: Id = -31;
pub const CACTUS: Id = -32;
pub const CABBAGE_PULT: Id = -33;
pub const COFFEE_BEAN: Id = -34;
pub const KERNEL_PULT: Id = -35;
pub const GARLIC: Id = -36;

pub const BOWLING_NUT: Id = -101;

pub const GRAVE: Id = -201;
pub const CRATER: Id = -202;
pub const ICE: Id = -203;

pub const BASIC_ZOMBIE: Id = 1;
pub const ROADCONE_ZOMBIE: Id = 2;
pub const BUCKET_ZOMBIE: Id = 3;
pub const FLAG_ZOMBIE: Id = 4;
pub const ALL_STAR_ZOMBIE: Id = 5;
pub const NEWSPAPER_ZOMBIE: Id = 6;
pub const SCREEN_DOOR_ZOMBIE: Id = 7;
pub const TRASHCAN_ZOMBIE: Id = 8;
pub const SNORKEL_ZOMBIE: Id = 9;
pub const ZOMBONI: Id = 10;
pub const DANCING_ZOMBIE: Id = 11;
pub const JITB_ZOMBIE: Id = 12;
pub const BALLOON_ZOMBIE: Id = 13;
pub const DIGGER_ZOMBIE: Id = 14;
pub const POGO_ZOMBIE: Id = 15;
pub const GARGANTUAR: Id = 16;

pub const HIDDEN_ZOMBIE: Id = 101;

pub const TRASHCAN: Id = 201;
pub const BACKUP_DANCER: Id = 202;
pub const IMP: Id = 203;

pub const LOGICAL_WIDTH: f32 = 1920.0;
pub const LOGICAL_HEIGHT: f32 = 1080.0;
pub const LOGICAL_BOUND: Vec2 = Vec2::new(LOGICAL_WIDTH * 1.0, LOGICAL_HEIGHT * 1.0);
pub const LOGICAL: Vec2 = Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
pub const SLOT_SIZE: Vec2 = Vec2::new(0.6, 0.7);
pub const BUTTON_SIZE: Vec2 = Vec2::new(SLOT_SIZE.x * 2.0, SLOT_SIZE.y);
pub const PROGRESS_SIZE: Vec2 = Vec2::new(1.5, 0.3);

pub const ACH_SIZE_FACTOR: f32 = 0.15;
pub const ACH_SIZE: Vec2 = Vec2::new(
    LOGICAL_WIDTH * ACH_SIZE_FACTOR * 2.0,
    LOGICAL_HEIGHT * ACH_SIZE_FACTOR * 2.0,
);

/// abs(delta z) must be below (hitbox1.height + hitbox2.height) / 2.0 / `COLLISION_Z_FACTOR`
pub const COLLISION_Z_FACTOR: f32 = 1.5;
/// "sparseness" is used in level::spawn module, where the probability increases by spawn turn,
/// while having a maximum cap of sparseness, and when chosen, the probability falls back to zero
pub const SPARSENESS: u32 = 255;
/// The percentage of parts above water of a zombie in the water
pub const WATER_PERCENTAGE: f32 = 0.7;
/// The distance in tiles that plants move down
pub const SHADOW_DISTANCE: f32 = 0.3;
/// Zoom factor for Egui
pub const UI_ZOOM_FACTOR: f32 = 1.0;

pub const ROOF_HIGHEST: usize = 5;
pub const ROOF_PIVOT: usize = 4;
pub const ROOF_SLOPE: f32 = 0.25;

/// This function defines the standard naming of creatures in configuration files and code
pub fn id_name(id: Id) -> &'static str {
    match id {
        0 => "default",

        PEASHOOTER => "peashooter",
        SUNFLOWER => "sunflower",
        CHERRY_BOMB => "cherry_bomb",
        WALL_NUT => "wall_nut",
        POTATO_MINE => "potato_mine",
        SNOW_PEA => "snow_pea",
        REPEATER => "repeater",
        ICEBERG_LETTUCE => "iceberg_lettuce",
        PUFF_SHROOM => "puff_shroom",
        SUN_SHROOM => "sun_shroom",
        GRAVE_BUSTER => "grave_buster",
        FUME_SHROOM => "fume_shroom",
        SCAREDY_SHROOM => "scaredy_shroom",
        ICE_SHROOM => "ice_shroom",
        DOOM_SHROOM => "doom_shroom",
        SUN_BEAN => "sun_bean",
        LILY_PAD => "lily_pad",
        SQUASH => "squash",
        THREEPEATER => "threepeater",
        TALL_NUT => "tall_nut",
        SPIKEWEED => "spikeweed",
        TORCHWOOD => "torchwood",
        BONK_CHOY => "bonk_choy",
        JALAPENO => "jalapeno",
        PLANTERN => "plantern",
        HURRIKALE => "hurrikale",
        FLOWER_POT => "flower_pot",
        PUMPKIN => "pumpkin",
        STARFRUIT => "starfruit",
        MAGNET_SHROOM => "magnet_shroom",
        BLOVER => "blover",
        CACTUS => "cactus",
        CABBAGE_PULT => "cabbage_pult",
        COFFEE_BEAN => "coffee_bean",
        KERNEL_PULT => "kernel_pult",
        GARLIC => "garlic",

        BOWLING_NUT => "bowling_nut",

        GRAVE => "grave",
        CRATER => "crater",

        BASIC_ZOMBIE => "basic",
        ROADCONE_ZOMBIE => "roadcone",
        BUCKET_ZOMBIE => "bucket",
        FLAG_ZOMBIE => "flag",
        ALL_STAR_ZOMBIE => "all_star",
        NEWSPAPER_ZOMBIE => "newspaper",
        SCREEN_DOOR_ZOMBIE => "screen_door",
        TRASHCAN_ZOMBIE => "trashcan",
        SNORKEL_ZOMBIE => "snorkel",
        ZOMBONI => "zomboni",
        DANCING_ZOMBIE => "dancing",
        JITB_ZOMBIE => "jitb",
        BALLOON_ZOMBIE => "balloon",
        DIGGER_ZOMBIE => "digger",
        POGO_ZOMBIE => "pogo",
        GARGANTUAR => "gargantuar",

        HIDDEN_ZOMBIE => "hidden",

        TRASHCAN => "trashcan_item",
        BACKUP_DANCER => "backup_dancer",
        IMP => "imp",

        _ => "unknown",
    }
}

/// This function returns the popularity of a creature
pub fn creature_popularity(creature: &game::Creature) -> f32 {
    1.0 / creature.cost as f32
}
