use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

pub(super) struct GamePositionPlugin;

impl Plugin for GamePositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Display>();
        app.init_resource::<Collision>();
        app.add_systems(
            PreUpdate,
            (update_collision,).run_if(in_state(info::GlobalStates::Play)),
        );
        app.add_systems(Update, (remove_outbound,));
        app.add_systems(
            PostUpdate,
            (
                update_transform,
                update_position,
                update_velocity,
                update_sprite,
            )
                .run_if(in_state(info::GlobalStates::Play)),
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

    pub fn regularize(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            r: self.r,
        }
    }

    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }

    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }

    pub fn z_i32(&self) -> i32 {
        self.z as i32
    }

    pub fn to_usize_pos(&self) -> (usize, usize) {
        let pos = self.regularize();
        (pos.x as usize, pos.y as usize)
    }

    pub fn same_y(&self, rhs: &Position) -> bool {
        self.y_i32() == rhs.y_i32()
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
impl HitBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Component, Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}
impl Velocity {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Self { x, y, z, r }
    }
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Gravity;

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

fn update_position(config: Res<config::Config>, mut q_pos: Query<(&Velocity, &mut Position)>) {
    q_pos.par_iter_mut().for_each(|(vel, mut pos)| {
        pos.x += vel.x * config.gamerule.speed.0;
        pos.y += vel.y * config.gamerule.speed.0;
        pos.z += vel.z * config.gamerule.speed.0;
        pos.r += vel.r * config.gamerule.speed.0;
    });
}

fn update_velocity(config: Res<config::Config>, mut q_vel: Query<&mut Velocity, With<Gravity>>) {
    q_vel.par_iter_mut().for_each(|mut vel| {
        vel.z += config.gamerule.gravity.0 * config.gamerule.speed.0;
    });
}

fn update_sprite(display: Res<Display>, mut q_pos: Query<(&HitBox, &mut Sprite), Changed<HitBox>>) {
    q_pos.par_iter_mut().for_each(|(hitbox, mut sprite)| {
        sprite.custom_size = Some(Vec2::from(hitbox) * display.ratio);
    });
}

fn update_collision(
    mut collision: ResMut<Collision>,
    config: Res<config::Config>,
    q_pos: Query<(Entity, &Position, &HitBox)>,
) {
    let map = Arc::new(RwLock::new(HashMap::new()));
    q_pos.par_iter().for_each(|(entity, pos, hitbox)| {
        let mut rng = rand::thread_rng();
        let ok = rng.gen_ratio(config.program.loss_rate.0 .0, config.program.loss_rate.0 .1);
        if ok {
            let set = q_pos
                .iter()
                .filter_map(|(sub_entity, sub_pos, sub_hitbox)| {
                    if sub_entity == entity
                        || (pos.y_i32() != sub_pos.y_i32())
                        || (pos.x - sub_pos.x).abs() >= (hitbox.width + sub_hitbox.width) / 2.0
                        || (pos.z - sub_pos.z).abs() >= (hitbox.height + sub_hitbox.height) / 2.0
                    {
                        None
                    } else {
                        Some(sub_entity)
                    }
                })
                .collect::<BTreeSet<_>>();
            map.write().unwrap().insert(entity, set);
        }
    });
    let mut to_remove = Vec::new();
    for (k, v) in map.read().unwrap().iter() {
        if v.is_empty() {
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
        if x < -LOGICAL_WIDTH || x > LOGICAL_WIDTH || y < -LOGICAL_HEIGHT || y > LOGICAL_HEIGHT {
            commands.entity(entity).despawn_recursive();
        }
    });
}
