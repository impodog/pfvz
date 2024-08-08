use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteItems {
    pub sun: Arc<sprite::FrameArr>,
    pub whack: Arc<sprite::FrameArr>,
}

pub(super) fn init_items(mut commands: Commands, server: Res<AssetServer>) {
    let items = SpriteItems {
        sun: super::load_animation(&server, "sprites/items/sun", Duration::from_millis(100)),
        whack: super::load_animation(&server, "sprites/items/whack", Duration::from_millis(100)),
    };
    commands.insert_resource(items);
}
