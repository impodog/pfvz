use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct GameVelocityPlugin;

impl Plugin for GameVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_position,
                update_bare_position,
                update_velocity,
                update_velocity_half,
                insert_velocity_base,
                modify_velocity_by_base,
            )
                .run_if(when_state!(play)),
        );
        app.add_systems(
            PostUpdate,
            (update_position_with_overlay,).run_if(when_state!(gaming)),
        );
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
impl std::ops::Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            r: self.r * rhs,
        }
    }
}
/// This component allows an object to fall in z coordinates due to gravity
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Gravity;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct GravityHalf;

fn update_position(
    time: Res<config::FrameTime>,
    mut q_pos: Query<(&Velocity, &mut game::LogicPosition), Without<game::Overlay>>,
) {
    q_pos.par_iter_mut().for_each(|(vel, mut pos)| {
        let factor = time.diff();
        pos.plus_assign(game::Position::new(
            vel.x * factor,
            vel.y * factor,
            vel.z * factor,
            vel.r * factor,
        ));
    });
}

fn update_position_with_overlay(
    time: Res<config::FrameTime>,
    mut q_pos: Query<(&game::Overlay, &Velocity, &mut game::LogicPosition)>,
) {
    q_pos.par_iter_mut().for_each(|(overlay, vel, mut pos)| {
        let factor = time.diff() * overlay.speed();
        pos.plus_assign(game::Position::new(
            vel.x * factor,
            vel.y * factor,
            vel.z * factor,
            vel.r * factor,
        ));
    });
}

fn update_bare_position(
    time: Res<config::FrameTime>,
    mut q_pos: Query<(&Velocity, &mut game::Position), Without<game::LogicPosition>>,
) {
    q_pos.par_iter_mut().for_each(|(vel, mut pos)| {
        let factor = time.diff();
        pos.x += vel.x * factor;
        pos.y += vel.y * factor;
        pos.z += vel.z * factor;
        pos.r += vel.r * factor;
    });
}

fn update_velocity(
    config: Res<config::Config>,
    time: Res<config::FrameTime>,
    mut q_vel: Query<&mut Velocity, With<Gravity>>,
) {
    q_vel.par_iter_mut().for_each(|mut vel| {
        vel.z -= time.diff() * config.gamerule.gravity.0;
    });
}

fn update_velocity_half(
    config: Res<config::Config>,
    time: Res<config::FrameTime>,
    mut q_vel: Query<&mut Velocity, With<GravityHalf>>,
) {
    q_vel.par_iter_mut().for_each(|mut vel| {
        vel.z -= time.diff() * config.gamerule.gravity.0 / 2.0;
    });
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct VelocityBase {
    base: Velocity,
    #[deref]
    pub factor: game::Factor,
}
impl VelocityBase {
    pub fn new(base: Velocity) -> Self {
        Self {
            base,
            factor: Default::default(),
        }
    }

    pub fn get(&self) -> Velocity {
        self.base * self.factor.factor()
    }

    pub fn get_without_factor(&self) -> &Velocity {
        &self.base
    }

    pub fn replace(&mut self, base: Velocity) -> Velocity {
        std::mem::replace(&mut self.base, base)
    }

    pub fn get_mut(&mut self) -> &mut Velocity {
        &mut self.base
    }
}

fn insert_velocity_base(
    commands: ParallelCommands,
    q_vel: Query<(Entity, Ref<Velocity>), Without<VelocityBase>>,
) {
    q_vel.iter().for_each(|(entity, velocity)| {
        if velocity.is_added() {
            commands.command_scope(|mut commands| {
                if let Some(mut commands) = commands.get_entity(entity) {
                    commands.insert(VelocityBase::new(*velocity));
                }
            });
        }
    })
}

fn modify_velocity_by_base(mut q_vel: Query<(&mut Velocity, Ref<VelocityBase>)>) {
    q_vel
        .par_iter_mut()
        .for_each(|(mut velocity, velocity_base)| {
            if !velocity_base.is_added() && velocity_base.is_changed() {
                *velocity = velocity_base.get();
            }
        });
}

pub trait ToVelocity: Send + Sync {
    fn to_velocity(&self) -> Velocity;
}

impl<T> ToVelocity for T
where
    T: Sized + Send + Sync + Clone + Copy + Into<Velocity>,
{
    fn to_velocity(&self) -> Velocity {
        (*self).into()
    }
}

#[derive(Clone)]
pub enum VelocityFunctor {
    Fixed(Velocity),
    Func(Arc<dyn ToVelocity>),
}
impl Default for VelocityFunctor {
    fn default() -> Self {
        Self::Fixed(Velocity::default())
    }
}
impl From<&VelocityFunctor> for Velocity {
    fn from(func: &VelocityFunctor) -> Velocity {
        match func {
            VelocityFunctor::Fixed(velocity) => *velocity,
            VelocityFunctor::Func(func) => func.to_velocity(),
        }
    }
}
impl<T> From<T> for VelocityFunctor
where
    T: ToVelocity + 'static,
{
    fn from(func: T) -> Self {
        Self::Func(Arc::new(func))
    }
}
