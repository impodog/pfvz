pub(crate) use crate::config::constants::*;
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

#[macro_export]
macro_rules! multiply_uf {
    ($x: expr, $y: expr) => {
        ($x as f32 * $y as f32) as u32
    };
}
