use crate::prelude::*;

pub(super) struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (update_animation,));
    }
}

#[derive(Debug)]
pub struct FrameArr {
    pub frames: Vec<Handle<Image>>,
    pub delta: Duration,
}

#[derive(Component, Debug, Clone)]
pub struct Animation {
    frames: Arc<FrameArr>,
    cursor: usize,
    begin: std::time::SystemTime,
}

impl Animation {
    pub fn new(frames: Arc<FrameArr>) -> Self {
        Self {
            frames,
            cursor: 0,
            begin: std::time::SystemTime::now(),
        }
    }
}

fn update_animation(mut q_anim: Query<(&mut Animation, &mut Handle<Image>)>) {
    let now = std::time::SystemTime::now();
    q_anim.par_iter_mut().for_each(|(mut anim, mut image)| {
        if now.duration_since(anim.begin).unwrap() >= anim.frames.delta {
            anim.cursor += 1;
            anim.begin = now;
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
