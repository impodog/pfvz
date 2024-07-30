use crate::prelude::*;

pub(super) struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (update_animation,));
    }
}

#[derive(Debug)]
pub struct FrameArr {
    pub frames: SmallVec<[Handle<Image>; 3]>,
    pub delta: Duration,
}

#[derive(Component, Debug, Clone)]
pub struct Animation {
    frames: Arc<FrameArr>,
    cursor: usize,
    timer: Timer,
}

impl Animation {
    pub fn new(frames: Arc<FrameArr>) -> Self {
        let timer = Timer::new(frames.delta, TimerMode::Repeating);
        Self {
            frames,
            cursor: 0,
            timer,
        }
    }

    pub fn replace(&mut self, frames: Arc<FrameArr>) {
        self.timer = Timer::new(frames.delta, TimerMode::Repeating);
        self.frames = frames;
        self.cursor = 0;
    }
}

fn update_animation(
    mut q_anim: Query<(Entity, &mut Animation, &mut Handle<Image>)>,
    q_overlay: Query<&game::Overlay>,
    q_parent: Query<&Parent>,
    time: Res<config::FrameTime>,
) {
    q_anim
        .par_iter_mut()
        .for_each(|(entity, mut anim, mut image)| {
            let delta = game::query_overlay(
                |overlay| {
                    if let Some(overlay) = overlay {
                        overlay.delta()
                    } else {
                        time.delta()
                    }
                },
                entity,
                &q_overlay,
                &q_parent,
            );
            anim.timer.tick(delta);
            if anim.timer.just_finished() {
                anim.cursor += 1;
                if anim.cursor >= anim.frames.frames.len() {
                    anim.cursor = 0;
                }
                *image = anim
                    .frames
                    .frames
                    .get(anim.cursor)
                    .expect("empty animation")
                    .clone();
            }
        });
}
