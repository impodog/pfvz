pub(crate) use crate::*;
pub(crate) use bevy::ecs::system::SystemId;
pub(crate) use bevy::prelude::*;
pub(crate) use lazy_static::{initialize, lazy_static};
pub(crate) use rand::Rng;
pub(crate) use std::collections::{BTreeSet, HashMap};
pub(crate) use std::sync::{Arc, RwLock};
pub(crate) use std::time::Duration;

// Positive ids for zombies, negative for plants/fungi
pub type Id = i32;

pub const PEASHOOTER: Id = -1;

pub const BASIC_ZOMBIE: Id = 1;

pub const LOGICAL_WIDTH: f32 = 1920.0;
pub const LOGICAL_HEIGHT: f32 = 1080.0;
pub const LOGICAL: Vec2 = Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
pub const SLOT_SIZE: Vec2 = Vec2::new(0.6, 0.8);

#[macro_export]
macro_rules! multiply_uf {
    ($x: expr, $y: expr) => {
        ($x as f32 * $y as f32) as u32
    };
}
