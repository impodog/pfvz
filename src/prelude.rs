pub(crate) use crate::*;
pub(crate) use bevy::ecs::system::SystemId;
pub(crate) use bevy::prelude::*;
pub(crate) use rand::Rng;
pub(crate) use std::collections::HashMap;
pub(crate) use std::sync::{Arc, RwLock};
pub(crate) use std::time::Duration;

/// Positive ids for plants/fungi; Negative ids for zombies
pub type Id = i32;

#[macro_export]
macro_rules! multiply {
    ($x: expr, $y: expr) => {
        ($x as f32 * $y as f32) as u32
    };
}
