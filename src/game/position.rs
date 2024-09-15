use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

pub(super) struct GamePositionPlugin;

impl Plugin for GamePositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Display>();
        app.init_resource::<Collision>();
        app.add_systems(PreUpdate, (update_collision,).run_if(when_state!(gaming)));
        app.add_systems(Update, (remove_outbound,));
        // Positioning system may be used by other states,
        // so it is not wrapped under play state
        app.add_systems(
            PostUpdate,
            (add_position, convert_position, update_transform).chain(),
        );
    }
}

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}
impl Position {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Self { x, y, z, r }
    }

    pub fn new_xy(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            z: 0.0,
            r: 0.0,
        }
    }

    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, r: 0.0 }
    }

    pub fn move_by(mut self, x: f32, y: f32) -> Self {
        self.x += x;
        self.y += y;
        self
    }

    pub fn move_z(mut self, z: f32) -> Self {
        self.z += z;
        self
    }

    pub fn distance_squared(&self, other: &Position) -> Orderedf32 {
        Orderedf32::from(
            (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2),
        )
    }
}
impl From<&Vec2> for Position {
    fn from(value: &Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0.0,
            r: 0.0,
        }
    }
}
impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            r: self.r + rhs.r,
        }
    }
}
impl std::ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            r: self.r - rhs.r,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PositionRange {
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub z: (f32, f32),
}
impl std::ops::Add<Position> for PositionRange {
    type Output = PositionRange;

    fn add(self, rhs: Position) -> Self::Output {
        Self::Output {
            x: (self.x.0 + rhs.x, self.x.1 + rhs.x),
            y: (self.y.0 + rhs.y, self.y.1 + rhs.y),
            z: (self.z.0 + rhs.z, self.z.1 + rhs.z),
        }
    }
}
impl Default for PositionRange {
    fn default() -> Self {
        game::PositionRange::new((0.0, f32::INFINITY), (-0.5, 0.5), (-0.01, 0.01))
    }
}
impl PositionRange {
    pub fn new(x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> Self {
        Self { x, y, z }
    }

    pub fn with_inf_z(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: (f32::NEG_INFINITY, f32::INFINITY),
        }
    }

    pub fn with_inverted_x(self) -> Self {
        Self {
            x: (-self.x.1, -self.x.0),
            y: self.y,
            z: self.z,
        }
    }

    fn intersects(x1: f32, x2: f32, y1: f32, y2: f32) -> bool {
        x1 <= y2 && y1 <= x2
    }

    pub fn merge(&mut self, range: &PositionRange) {
        self.x.0 = self.x.0.min(range.x.0);
        self.x.1 = self.x.1.max(range.x.1);
        self.y.0 = self.y.0.min(range.y.0);
        self.y.1 = self.y.1.max(range.y.1);
        self.z.0 = self.z.0.min(range.z.0);
        self.z.1 = self.z.1.max(range.z.1);
    }

    pub fn contains(&self, pos: &Position, hitbox: &HitBox) -> bool {
        Self::intersects(
            self.x.0,
            self.x.1,
            pos.x - hitbox.width / 2.0,
            pos.x + hitbox.width / 2.0,
        ) && Self::intersects(
            self.z.0,
            self.z.1,
            pos.z - hitbox.height / 2.0,
            pos.z + hitbox.height / 2.0,
        ) && self.y.0 <= pos.y
            && pos.y <= self.y.1
    }
}

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct HitBox {
    pub width: f32,
    pub height: f32,
}
impl From<&HitBox> for Vec2 {
    fn from(value: &HitBox) -> Self {
        Self::new(value.width, value.height)
    }
}
impl From<&Vec2> for HitBox {
    fn from(value: &Vec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}
impl std::ops::Mul<f32> for HitBox {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}
impl HitBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn with_width(self, width: f32) -> Self {
        Self {
            width,
            height: self.height,
        }
    }

    pub fn with_height(self, height: f32) -> Self {
        Self {
            width: self.width,
            height,
        }
    }

    pub fn with_height_multiply(self, factor: f32) -> Self {
        Self {
            width: self.width,
            height: self.height * factor,
        }
    }
}

// This component marks that one entity should never be skipped while loss calculation
#[derive(Component, Default, Clone, Copy)]
pub struct NoCollisionLoss;

#[derive(Resource, Default, Debug)]
pub struct Collision {
    map: HashMap<Entity, BTreeSet<Entity>>,
}
impl Collision {
    pub fn test(&self, lhs: &Entity, rhs: &Entity) -> bool {
        self.map.get(lhs).is_some_and(|set| set.contains(rhs))
    }

    pub fn get(&self, entity: &Entity) -> Option<&BTreeSet<Entity>> {
        self.map.get(entity)
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct Display {
    pub ratio: f32,
}

fn update_transform(
    display: Res<Display>,
    mut q_pos: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    q_pos.par_iter_mut().for_each(|(pos, mut transform)| {
        transform.translation.x = pos.x * display.ratio;
        transform.translation.y = (pos.y + pos.z) * display.ratio;
        transform.rotation = Quat::from_rotation_z(pos.r);
    });
}

fn add_position(
    mut commands: Commands,
    q_pos: Query<
        (Entity, &game::LogicPosition, &game::HitBox),
        (Added<game::LogicPosition>, Without<game::Position>),
    >,
) {
    q_pos.iter().for_each(|(entity, logic, hitbox)| {
        commands.entity(entity).try_insert(logic.center_of(hitbox));
    });
}

fn convert_position(mut q_pos: Query<(&mut game::Position, &game::HitBox, &game::LogicPosition)>) {
    q_pos.par_iter_mut().for_each(|(mut pos, hitbox, logic)| {
        *pos = logic.center_of(hitbox);
    });
}

fn update_collision(
    mut collision: ResMut<Collision>,
    config: Res<config::Config>,
    q_pos: Query<(Entity, &Position, &HitBox), Without<Parent>>,
    q_no_loss: Query<&NoCollisionLoss>,
    level: Res<level::Level>,
    mut commands: Commands,
) {
    let map = Arc::new(RwLock::new(HashMap::new()));
    q_pos.par_iter().for_each(|(entity, pos, hitbox)| {
        let mut rng = rand::thread_rng();
        // With `NoCollisionLoss`, the loss calculation is skipped
        let no_loss = q_no_loss.get(entity).is_ok();
        let ok =
            no_loss || rng.gen_ratio(config.program.loss_rate.0 .0, config.program.loss_rate.0 .1);
        if ok {
            let set = q_pos
                .iter()
                .filter_map(|(sub_entity, sub_pos, sub_hitbox)| {
                    // NOTE: Regularized positions for y comparison
                    if no_loss {
                        // No loss allow a computation of 2d geometry
                        if sub_entity == entity
                            || (pos.x - sub_pos.x).abs() >= (hitbox.width + sub_hitbox.width) / 2.0
                            || (pos.z + pos.y - sub_pos.z - sub_pos.y).abs()
                                >= (hitbox.height + sub_hitbox.height) / 2.5
                        {
                            None
                        } else {
                            Some(sub_entity)
                        }
                    } else {
                        #[allow(clippy::collapsible_else_if)]
                        if !level.config.layout.same_y(pos, sub_pos)
                            || sub_entity == entity
                            || (pos.x - sub_pos.x).abs() >= (hitbox.width + sub_hitbox.width) / 2.0
                            || (pos.z - sub_pos.z).abs()
                                >= (hitbox.height + sub_hitbox.height) / 2.0 / COLLISION_Z_FACTOR
                        {
                            None
                        } else {
                            Some(sub_entity)
                        }
                    }
                })
                .collect::<BTreeSet<_>>();
            map.write().unwrap().insert(entity, set);
        }
    });
    let mut to_remove = Vec::new();
    for (k, v) in map.read().unwrap().iter() {
        if v.is_empty() || commands.get_entity(*k).is_none() {
            to_remove.push(*k);
        }
    }
    for k in to_remove.into_iter() {
        map.write().unwrap().remove(&k);
    }
    let map = Arc::into_inner(map).unwrap();
    let map = RwLock::into_inner(map).unwrap();
    let _ = std::mem::replace(&mut collision.map, map);
}

fn remove_outbound(
    mut commands: Commands,
    display: Res<Display>,
    q_pos: Query<(Entity, &Position), Changed<Position>>,
) {
    q_pos.iter().for_each(|(entity, pos)| {
        let x = pos.x * display.ratio;
        let y = pos.y * display.ratio;
        if !(-LOGICAL_BOUND.x..=LOGICAL_BOUND.x).contains(&x)
            || !(-LOGICAL_BOUND.y..=LOGICAL_BOUND.y).contains(&y)
        {
            commands.entity(entity).despawn_recursive();
        }
    });
}
