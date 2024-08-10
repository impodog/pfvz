use crate::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SpriteLayout(Vec<Handle<Image>>);
impl SpriteLayout {
    fn load(server: &Res<AssetServer>, base: &str) -> Self {
        let mut v = Vec::new();
        for index in 1usize.. {
            let path = format!("assets/{}/{}.png", base, index);
            if std::path::Path::new(&path).try_exists().unwrap_or_default() {
                v.push(server.load(format!("{}/{}.png", base, index)));
            } else {
                break;
            }
        }
        Self(v)
    }
}

#[derive(Resource)]
pub struct SpriteLayouts {
    pub day: SpriteLayout,
    pub night: SpriteLayout,
    pub pool: SpriteLayout,
}
impl SpriteLayouts {
    pub fn get(&self, layout: &level::LayoutKind) -> &SpriteLayout {
        match layout {
            level::LayoutKind::Day => &self.day,
            level::LayoutKind::Night => &self.night,
            level::LayoutKind::Pool => &self.pool,
        }
    }
}

pub(super) fn init_layouts(mut commands: Commands, server: Res<AssetServer>) {
    let layouts = SpriteLayouts {
        day: SpriteLayout::load(&server, "sprites/layouts/day"),
        night: SpriteLayout::load(&server, "sprites/layouts/night"),
        pool: SpriteLayout::load(&server, "sprites/layouts/pool"),
    };
    commands.insert_resource(layouts);
}
