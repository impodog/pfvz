pub(crate) use crate::config::constants::*;
pub(crate) use crate::*;
pub(crate) use bevy::ecs::system::SystemId;
pub(crate) use bevy::prelude::*;
pub(crate) use bevy::sprite::Anchor;
pub(crate) use bitflags::bitflags;
pub(crate) use lazy_static::{initialize, lazy_static};
pub(crate) use ordered_float::OrderedFloat;
pub(crate) use rand::distributions::WeightedIndex;
pub(crate) use rand::prelude::*;
pub(crate) use smallvec::SmallVec;
pub(crate) use std::collections::{BTreeMap, BTreeSet, HashMap};
pub(crate) use std::ops::Range;
pub(crate) use std::sync::{Arc, RwLock};
pub(crate) use std::time::Duration;

// Positive ids for zombies, negative for plants/fungi
pub type Id = i32;
pub(crate) type Orderedf32 = OrderedFloat<f32>;

#[macro_export]
macro_rules! multiply_uf {
    ($x: expr, $y: expr) => {
        ($x as f32 * $y as f32) as u32
    };
}
