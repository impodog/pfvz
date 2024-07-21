use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Wave {
    pub points: u32,
    pub big: bool,
    pub fixed: Vec<(Id, usize)>,
    pub avail: Vec<(Id, usize)>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum LayoutKind {
    #[default]
    Day,
}
impl LayoutKind {
    pub const fn size(&self) -> (usize, usize) {
        match self {
            Self::Day => (9, 5),
        }
    }

    pub const fn lane_size(&self) -> usize {
        self.size().0
    }

    pub const fn size_f32(&self) -> (f32, f32) {
        let s = self.size();
        (s.0 as f32, s.1 as f32)
    }

    /// This returns a index applicable to `PlantLayout`, or usize::MAX if conversion is not
    /// possible
    pub fn position_to_index(&self, pos: &game::Position) -> usize {
        let size = self.size();
        let x = pos.x_i32() + (size.0 as i32 / 2);
        let y = pos.y_i32() + (size.1 as i32 / 2);
        if let Ok(x) = usize::try_from(x) {
            if let Ok(y) = usize::try_from(y) {
                return y * size.0 + x;
            }
        }
        usize::MAX
    }

    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Day => true,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum GameKind {
    #[default]
    Adventure,
}
impl GameKind {
    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Adventure => true,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum SelectionArr {
    #[default]
    Any,
    Few(Vec<Id>),
    All(Vec<Id>),
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LevelConfig {
    pub layout: LayoutKind,
    pub game: GameKind,
    pub selection: SelectionArr,
    pub sun: u32,
}
impl LevelConfig {
    pub fn is_sun_spawn(&self) -> bool {
        self.layout.is_sun_spawn() && self.game.is_sun_spawn()
    }
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct Level {
    pub waves: Vec<Wave>,
    pub config: LevelConfig,
}
