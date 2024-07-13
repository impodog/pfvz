pub(crate) use crate::*;
pub(crate) use bevy::ecs::system::SystemId;
pub(crate) use bevy::prelude::*;
pub(crate) use lazy_static::{initialize, lazy_static};
pub(crate) use rand::Rng;
pub(crate) use std::collections::HashMap;
pub(crate) use std::sync::{Arc, RwLock};
pub(crate) use std::time::Duration;

/// Negative ids for plants/fungi; Positive ids for zombies
pub type Id = i32;
pub const LOGICAL_WIDTH: f32 = 1920.0;
pub const LOGICAL_HEIGHT: f32 = 1080.0;
pub const LOGICAL: Vec2 = Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
pub const SLOT_SIZE: Vec2 = Vec2::new(0.5, 0.8);

#[macro_export]
macro_rules! multiply {
    ($x: expr, $y: expr) => {
        ($x as f32 * $y as f32) as u32
    };
}
