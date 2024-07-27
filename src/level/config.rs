use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Wave {
    pub points: u32,
    pub wait: f32,
    pub big: bool,
    /// These zombies will be spawned at highest priority (no points cost)
    pub fixed: Vec<(Id, usize)>,
    /// If there are spare points, spawn these by equal chance
    pub avail: Vec<Id>,
}

#[derive(
    Serialize, Deserialize, Resource, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct LevelIndex {
    pub stage: u8,
    pub level: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum TileFeature {
    Grass,
    Dirt,
}
impl TileFeature {
    pub fn compat(&self, creature: &game::Creature) -> bool {
        // TODO: Add compatible checks
        true
    }
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

    pub const fn size_vec2(&self) -> Vec2 {
        let s = self.size_f32();
        Vec2::new(s.0, s.1)
    }

    pub fn half_size_f32(&self) -> (f32, f32) {
        let s = self.size();
        (s.0 as f32 / 2.0, s.1 as f32 / 2.0)
    }

    pub fn half_size_vec2(&self) -> Vec2 {
        let s = self.half_size_f32();
        Vec2::new(s.0, s.1)
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

    pub fn get_tile(&self, x: usize, y: usize) -> TileFeature {
        match self {
            Self::Day => TileFeature::Grass,
        }
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
impl SelectionArr {
    pub fn modify(&self, selection: &mut game::Selection) {
        match self {
            Self::Any => {
                selection.0.clear();
            }
            Self::Few(ids) | Self::All(ids) => {
                selection.0 = ids.clone();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct StateModify {
    // TODO: Make this 0 to give default item(money)
    pub give: Id,
    pub next: LevelIndex,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LevelConfig {
    pub layout: LayoutKind,
    pub game: GameKind,
    pub selection: SelectionArr,
    #[serde(default)]
    pub modify: Option<StateModify>,
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
