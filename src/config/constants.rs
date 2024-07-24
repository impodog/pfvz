use crate::prelude::*;

pub const PEASHOOTER: Id = -1;
pub const SUNFLOWER: Id = -2;
pub const CHERRY_BOMB: Id = -3;
pub const WALL_NUT: Id = -4;

pub const BASIC_ZOMBIE: Id = 1;
pub const ROADCONE_ZOMBIE: Id = 2;
pub const BUCKET_ZOMBIE: Id = 3;
pub const FLAG_ZOMBIE: Id = 4;

pub const LOGICAL_WIDTH: f32 = 1920.0;
pub const LOGICAL_HEIGHT: f32 = 1080.0;
pub const LOGICAL: Vec2 = Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
pub const SLOT_SIZE: Vec2 = Vec2::new(0.6, 0.8);
pub const PROGRESS_SIZE: Vec2 = Vec2::new(1.5, 0.3);
