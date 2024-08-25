use crate::prelude::*;

pub(super) struct CompnDivertPlugin;

impl Plugin for CompnDivertPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (divert_work,));
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum DivertStatus {
    #[default]
    Start,
    Working,
}

#[derive(Default, Component, Debug, Clone)]
pub struct Divert {
    y: f32,
    velocity: f32,
    status: DivertStatus,
}
impl Divert {
    pub fn new(y: f32, velocity: f32) -> Self {
        Self {
            y,
            velocity,
            ..Default::default()
        }
    }
}

fn divert_work(
    commands: ParallelCommands,
    mut q_divert: Query<(
        Entity,
        &game::Overlay,
        &mut game::LogicPosition,
        &mut Divert,
    )>,
    time: Res<config::FrameTime>,
) {
    q_divert
        .par_iter_mut()
        .for_each(|(entity, overlay, mut logic, mut divert)| {
            match divert.status {
                DivertStatus::Start => {
                    divert.velocity = divert.velocity.copysign(divert.y - logic.base_raw().y);
                    divert.status = DivertStatus::Working;
                }
                DivertStatus::Working => {}
            }
            if (logic.base_raw().y > divert.y) ^ (divert.velocity <= 0.0) {
                logic.base_raw_mut().y = divert.y;
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(entity) {
                        commands.remove::<Divert>();
                    }
                });
            } else {
                let factor = time.diff() * overlay.speed();
                logic.base_raw_mut().y += factor * divert.velocity;
            }
        });
}
