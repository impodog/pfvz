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
    Serialize,
    Deserialize,
    Resource,
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct LevelIndex {
    pub stage: u8,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileFeature {
    Grass,
    Dirt,
    Water,
}
impl TileFeature {
    pub fn is_compat(&self, creature: &game::Creature) -> bool {
        creature.flags.is_compat(self.flags())
    }

    pub fn flags(&self) -> level::CreatureFlags {
        match self {
            Self::Grass => level::CreatureFlags::MAKE_TERRESTRIAL,
            Self::Dirt => level::CreatureFlags::MAKE_BARE_GROUND,
            Self::Water => level::CreatureFlags::MAKE_AQUATIC,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum LayoutKind {
    #[default]
    Day,
    Night,
    Pool,
}
impl LayoutKind {
    pub const fn size(&self) -> (usize, usize) {
        match self {
            Self::Day => (9, 5),
            Self::Night => (9, 5),
            Self::Pool => (9, 6),
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

    pub fn get_tile(&self, x: usize, y: usize) -> TileFeature {
        match self {
            Self::Day => TileFeature::Grass,
            Self::Night => TileFeature::Grass,
            Self::Pool => {
                if y == 2 || y == 3 {
                    TileFeature::Water
                } else {
                    TileFeature::Grass
                }
            }
        }
    }

    /// Get a tile that represents the lane's feature
    /// This is used in zombie spawning
    pub fn get_lane(&self, y: usize) -> TileFeature {
        match self {
            Self::Day => TileFeature::Grass,
            Self::Night => TileFeature::Grass,
            Self::Pool => {
                if y == 2 || y == 3 {
                    TileFeature::Water
                } else {
                    TileFeature::Grass
                }
            }
        }
    }

    /// Get the tile's picture index in the loaded layout vector
    pub fn get_layout(&self, x: usize, y: usize) -> usize {
        let size = self.size();
        match self {
            Self::Day | Self::Night => {
                let pos = y * size.0 + x;
                pos % 2
            }
            Self::Pool => {
                if y == 2 || y == 3 {
                    2
                } else {
                    let pos = y * size.0 + x;
                    pos % 2
                }
            }
        }
    }

    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Day => true,
            Self::Night => false,
            Self::Pool => true,
        }
    }

    fn has_grave(&self) -> bool {
        match self {
            Self::Day => false,
            Self::Night => true,
            Self::Pool => false,
        }
    }

    pub fn is_night(&self) -> bool {
        match self {
            Self::Day => false,
            Self::Night => true,
            Self::Pool => false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameKind {
    #[default]
    Adventure,
    WhackAZombie,
}
impl GameKind {
    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Adventure => true,
            Self::WhackAZombie => false,
        }
    }

    fn has_grave(&self) -> bool {
        match self {
            Self::Adventure => true,
            Self::WhackAZombie => true,
        }
    }

    fn is_compat(&self, id: Id) -> bool {
        // TODO: ban some plants according to the game kind
        true
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

    pub fn len(&self) -> usize {
        match self {
            Self::Any => 0,
            Self::Few(v) | Self::All(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Any)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct StateModify {
    // TODO: Make this 0 to give default item(money)
    pub give: Id,
    pub next: LevelIndex,
    #[serde(default)]
    pub slots: usize,
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

    pub fn has_grave(&self) -> bool {
        self.layout.has_grave() && self.game.has_grave()
    }

    pub fn is_compat(&self, id: Id) -> bool {
        self.game.is_compat(id)
    }

    pub fn max_select(&self, slots: usize) -> usize {
        match &self.selection {
            SelectionArr::Any => slots,
            SelectionArr::Few(v) => slots.saturating_sub(v.len()),
            SelectionArr::All(_) => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct Level {
    pub waves: Vec<Wave>,
    pub config: LevelConfig,
}
