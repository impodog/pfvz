use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct VelocityX(pub f32);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct VelocityXRange(pub (f32, f32));

impl From<VelocityX> for game::Velocity {
    fn from(value: VelocityX) -> Self {
        Self::new(value.0, 0.0, 0.0, 0.0)
    }
}
impl From<VelocityXRange> for game::Velocity {
    fn from(value: VelocityXRange) -> Self {
        Self::new(
            rand::thread_rng().gen_range(value.0 .0..=value.0 .1),
            0.0,
            0.0,
            0.0,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct VelocityAny {
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub z: f32,
    pub r: f32,
}
impl From<VelocityAny> for game::Velocity {
    fn from(value: VelocityAny) -> Self {
        Self::new(
            rand::thread_rng().gen_range(value.x.0..=value.x.1),
            rand::thread_rng().gen_range(value.y.0..=value.y.1),
            value.z,
            value.r,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PositionRangeX(pub f32);
impl From<PositionRangeX> for game::PositionRange {
    fn from(value: PositionRangeX) -> Self {
        let mut range = game::PositionRange::default();
        range.x.end = value.0;
        range
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PositionRangeXStartEnd(pub f32, pub f32);
impl From<PositionRangeXStartEnd> for game::PositionRange {
    fn from(value: PositionRangeXStartEnd) -> Self {
        let mut range = game::PositionRange::default();
        range.x.start = value.0;
        range.x.end = value.1;
        range
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PositionRangeXY(pub f32, pub f32);
impl From<PositionRangeXY> for game::PositionRange {
    fn from(value: PositionRangeXY) -> Self {
        let mut range = game::PositionRange::default();
        range.x.end = value.0;
        range.y.end = value.1;
        range
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DurationRangeSecs(pub f32, pub f32);
impl From<DurationRangeSecs> for Duration {
    fn from(value: DurationRangeSecs) -> Self {
        Duration::from_secs_f32(rand::thread_rng().gen_range(value.0..=value.1))
    }
}
