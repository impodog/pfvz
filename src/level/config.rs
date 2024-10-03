use crate::prelude::*;
use level::util::BTreeMapSerde;
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

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LevelIndex {
    pub stage: u32,
    pub level: u32,
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayoutKind {
    #[default]
    Day,
    Night,
    Pool,
    Fog,
    Roof,
}
impl LayoutKind {
    pub const fn size(&self) -> (usize, usize) {
        match self {
            Self::Day => (9, 5),
            Self::Night => (9, 5),
            Self::Pool => (9, 6),
            Self::Fog => (9, 6),
            Self::Roof => (9, 5),
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
            Self::Pool | Self::Fog => {
                if (y == 2 || y == 3) && (0..=8).contains(&x) {
                    TileFeature::Water
                } else {
                    TileFeature::Grass
                }
            }
            Self::Roof => TileFeature::Dirt,
        }
    }

    /// Get a tile that represents the lane's feature
    /// This is used in zombie spawning
    pub fn get_lane(&self, y: usize) -> TileFeature {
        match self {
            Self::Day => TileFeature::Grass,
            Self::Night => TileFeature::Grass,
            Self::Pool | Self::Fog => {
                if y == 2 || y == 3 {
                    TileFeature::Water
                } else {
                    TileFeature::Grass
                }
            }
            Self::Roof => TileFeature::Grass,
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
            Self::Pool | Self::Fog => {
                if y == 2 || y == 3 {
                    2
                } else {
                    let pos = y * size.0 + x;
                    pos % 2
                }
            }
            Self::Roof => {
                let pos = y * size.0 + x;
                if x < ROOF_HIGHEST {
                    pos % 2 + 2
                } else {
                    pos % 2
                }
            }
        }
    }

    /// Get the z-axis depth of a row
    pub fn get_disp(&self, x: usize) -> f32 {
        match self {
            Self::Day | Self::Night | Self::Pool | Self::Fog => 0.0,
            Self::Roof => (x.min(ROOF_HIGHEST) as f32 - ROOF_PIVOT as f32) * ROOF_SLOPE,
        }
    }

    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Day => true,
            Self::Night => false,
            Self::Pool => true,
            Self::Fog => false,
            Self::Roof => true,
        }
    }

    fn has_grave(&self) -> bool {
        match self {
            Self::Day => false,
            Self::Night => true,
            Self::Pool => false,
            Self::Fog => false,
            Self::Roof => false,
        }
    }

    pub fn is_night(&self) -> bool {
        match self {
            Self::Day => false,
            Self::Night => true,
            Self::Pool => false,
            Self::Fog => true,
            Self::Roof => false,
        }
    }

    fn has_fog(&self) -> bool {
        matches!(self, Self::Fog)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameKind {
    #[default]
    Adventure,
    Intro,
    WhackAZombie,
    QuickShot,
    Random,
    Thunder,
    Columns,
    InfiSun,
    NoSun,
    Zomboss,
}
impl GameKind {
    fn is_sun_spawn(&self) -> bool {
        match self {
            Self::Adventure => true,
            Self::Intro => false,
            Self::WhackAZombie => false,
            Self::QuickShot => true,
            Self::Random => true,
            Self::Thunder => true,
            Self::Columns => false,
            Self::InfiSun => true,
            Self::NoSun => false,
            Self::Zomboss => true,
        }
    }

    fn has_grave(&self) -> bool {
        match self {
            Self::Adventure => true,
            Self::Intro => false,
            Self::WhackAZombie => true,
            Self::QuickShot => true,
            Self::Random => true,
            Self::Thunder => true,
            Self::Columns => true,
            Self::InfiSun => true,
            Self::NoSun => true,
            Self::Zomboss => true,
        }
    }

    fn is_compat(&self, id: Id) -> bool {
        match self {
            Self::NoSun => !matches!(
                id,
                SUNFLOWER | SUN_SHROOM | SUN_BEAN | ETHYLENE | GOLD_BLOOM | TWIN_SUNFLOWER
            ),
            _ => true,
        }
    }

    fn has_fog(&self) -> bool {
        !matches!(self, GameKind::Thunder)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Deref, DerefMut, PartialEq, Eq)]
pub struct GameKindSet(BTreeSet<GameKind>);
impl Default for GameKindSet {
    fn default() -> Self {
        Self(BTreeSet::from_iter([GameKind::default()]))
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum SelectionArr {
    #[default]
    Any,
    ThisMany(usize),
    Few(Vec<Id>),
    All(Vec<Id>),
}
impl SelectionArr {
    pub fn modify(&self, selection: &mut game::Selection) {
        match self {
            Self::Any | Self::ThisMany(_) => {
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
            Self::ThisMany(len) => *len,
            Self::Few(v) | Self::All(v) => v.len(),
        }
    }

    pub fn slots(&self, slots: usize) -> usize {
        match self {
            Self::Any => slots,
            Self::ThisMany(len) => *len,
            Self::Few(v) => v.len().max(slots),
            Self::All(v) => v.len(),
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
    #[serde(default)]
    pub unlock: Vec<LevelIndex>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LevelPublish {
    pub name: Option<String>,
    pub creator: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LevelConveyorItem {
    pub max: usize,
    pub weight: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelConveyor {
    pub items: BTreeMapSerde<Id, LevelConveyorItem>,
    pub interval: f32,
    pub speed: f32,
}
impl Default for LevelConveyor {
    fn default() -> Self {
        Self {
            items: Default::default(),
            interval: 10.0,
            speed: 0.25,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LevelConfig {
    pub layout: LayoutKind,
    #[serde(default)]
    pub game: GameKindSet,
    pub selection: SelectionArr,
    #[serde(default)]
    pub modify: Option<StateModify>,
    #[serde(default)]
    pub bgm: Option<String>,
    pub sun: u32,
    #[serde(default)]
    pub publish: LevelPublish,
}
impl LevelConfig {
    pub fn is_sun_spawn(&self) -> bool {
        self.layout.is_sun_spawn() && self.game.iter().all(GameKind::is_sun_spawn)
    }

    pub fn has_grave(&self) -> bool {
        self.layout.has_grave() && self.game.iter().all(GameKind::has_grave)
    }

    pub fn is_compat(&self, id: Id) -> bool {
        self.game.iter().all(|game| game.is_compat(id))
    }

    pub fn has_fog(&self) -> bool {
        self.layout.has_fog() && self.game.iter().all(GameKind::has_fog)
    }

    pub fn max_select(&self, slots: usize) -> usize {
        match &self.selection {
            SelectionArr::Any => slots,
            SelectionArr::ThisMany(slots) => *slots,
            SelectionArr::Few(v) => slots.saturating_sub(v.len()),
            SelectionArr::All(_) => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Resource, Default, Debug, Clone)]
pub struct Level {
    pub waves: Vec<Wave>,
    pub config: LevelConfig,
    #[serde(default)]
    pub conveyor: Option<LevelConveyor>,
    #[serde(default)]
    pub zomboss: Option<zombies::ZombossConfig>,
}
impl Level {
    pub fn hide_sun(&self) -> bool {
        self.conveyor.is_some()
    }
}
