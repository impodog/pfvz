use crate::prelude::*;

pub(super) struct AssetsSpritesPlugin;

impl Plugin for AssetsSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_chunks, init_layouts));
    }
}

fn load_animation(server: Res<AssetServer>, base: &str, delta: Duration) -> sprite::FrameArr {
    let mut i = 1;
    let mut frames = Vec::new();
    loop {
        let path = format!("{}{}.png", base, i);
        if std::fs::exists(&path).unwrap_or(false) {
            let image: Handle<Image> = server.load(path);
            frames.push(image);
            i += 1;
        } else {
            break;
        }
    }
    sprite::FrameArr { frames, delta }
}

#[derive(Resource)]
pub struct SpriteChunks {
    pub pvfz: Handle<Image>,
}

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
}
impl SpriteLayouts {
    pub fn get(&self, layout: &level::LayoutKind) -> &SpriteLayout {
        match layout {
            level::LayoutKind::Day => &self.day,
        }
    }
}

fn init_chunks(mut commands: Commands, server: Res<AssetServer>) {
    let chunks = SpriteChunks {
        pvfz: server.load("sprites/chunks/pfvz.png"),
    };
    commands.insert_resource(chunks);
}

fn init_layouts(mut commands: Commands, server: Res<AssetServer>) {
    let layouts = SpriteLayouts {
        day: SpriteLayout::load(&server, "sprites/layouts/day"),
    };
    commands.insert_resource(layouts);
}
