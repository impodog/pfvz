mod chunks;
mod items;
mod layouts;
mod plants;
mod zombies;

pub use chunks::*;
pub use items::*;
pub use layouts::*;
pub use plants::*;
pub use zombies::*;

use crate::prelude::*;

pub(super) struct AssetsSpritesPlugin;

impl Plugin for AssetsSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                init_chunks,
                init_layouts,
                init_plants,
                init_zombies,
                init_items,
            ),
        );
    }
}

pub(super) fn load_animation_then<F>(
    server: &Res<AssetServer>,
    base: &str,
    delta: Duration,
    f: F,
) -> Arc<sprite::FrameArr>
where
    F: FnOnce(sprite::FrameArr) -> sprite::FrameArr,
{
    let mut i = 1;
    let mut frames = SmallVec::new();
    loop {
        let path = format!("{}/{}.png", base, i);
        if std::fs::File::open(format!("assets/{}", path)).is_ok() {
            let image: Handle<Image> = server.load(path);
            frames.push(image);
            i += 1;
        } else {
            break;
        }
    }
    if i <= 1 {
        error!(
            "Frame array is empty when loading {:?}. This will crash!",
            base
        );
    }
    Arc::new(f(sprite::FrameArr { frames, delta }))
}

/// Load an animation from directory
pub(super) fn load_animation(
    server: &Res<AssetServer>,
    base: &str,
    delta: Duration,
) -> Arc<sprite::FrameArr> {
    load_animation_then(server, base, delta, |frames| frames)
}
