use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteLayout {
    pub grass: Vec<Handle<Image>>,
}
impl SpriteLayout {
    fn load(server: &Res<AssetServer>, base: &str) -> Self {
        Self {
            grass: vec![
                server.load(format!("{}/grassl.png", base)),
                server.load(format!("{}/grassd.png", base)),
            ],
        }
    }
}

#[derive(Resource)]
pub struct SpriteLayouts {
    pub day: SpriteLayout,
    pub night: SpriteLayout,
}
impl SpriteLayouts {
    pub fn get(&self, layout: &level::LayoutKind) -> &SpriteLayout {
        match layout {
            level::LayoutKind::Day => &self.day,
            level::LayoutKind::Night => &self.night,
        }
    }
}

pub(super) fn init_layouts(mut commands: Commands, server: Res<AssetServer>) {
    let layouts = SpriteLayouts {
        day: SpriteLayout::load(&server, "sprites/layouts/day"),
        night: SpriteLayout::load(&server, "sprites/layouts/night"),
    };
    commands.insert_resource(layouts);
}
