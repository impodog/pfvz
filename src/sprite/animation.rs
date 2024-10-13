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
impl Default for FrameArr {
    fn default() -> Self {
        Self {
            // This additional blank is added to prevent issues caused by empty animations
            frames: smallvec::smallvec![Default::default()],
            delta: Default::default(),
        }
    }
}
impl FrameArr {
    /// Append the reversed frames to the end of the array, doubling is size
    pub fn then_reverse(mut self) -> FrameArr {
        let frames = self.frames.clone();
        self.frames.extend(frames.into_iter().rev());
        self
    }
}

#[derive(Component, Debug, Clone)]
pub struct Animation {
    frames: Arc<FrameArr>,
    cursor: usize,
    timer: Timer,
    added: bool,
}

impl Animation {
    pub fn new(frames: Arc<FrameArr>) -> Self {
        let timer = Timer::new(frames.delta, TimerMode::Repeating);
        Self {
            frames,
            cursor: 0,
            timer,
            added: true,
        }
    }

    /// Replace to the new frames immediately, may cause time of animation shorter
    pub fn replace(&mut self, frames: Arc<FrameArr>) {
        self.reset(frames);
        self.timer.set_elapsed(self.timer.duration());
    }

    /// Replace to the new frames, then reset the timer, so that the animation is fully played
    pub fn reset(&mut self, frames: Arc<FrameArr>) {
        self.timer = Timer::new(frames.delta, TimerMode::Repeating);
        self.frames = frames;
        self.cursor = 0;
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished() && self.cursor == self.frames.frames.len() - 1
    }

    pub fn frames(&self) -> Arc<FrameArr> {
        self.frames.clone()
    }
}

fn update_animation(
    mut q_anim: Query<(Entity, &mut Animation, &mut Handle<Image>)>,
    q_overlay: Query<&game::Overlay>,
    q_parent: Query<&Parent>,
    chunks: Res<assets::SpriteChunks>,
    time: Res<config::FrameTime>,
) {
    q_anim
        .par_iter_mut()
        .for_each(|(entity, mut anim, mut image)| {
            if anim.is_added() {
                *image = chunks.transparent.clone();
                return;
            }
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
            // is_added prevents white chunks showing up
            if anim.added || anim.timer.just_finished() {
                anim.cursor += 1;
                anim.added = false;
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
