use crate::prelude::*;

pub(super) struct AssetsSpritesPlugin;

impl Plugin for AssetsSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_sprites,));
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

fn init_sprites(mut commands: Commands, server: Res<AssetServer>) {
    let chunks = SpriteChunks {
        pvfz: server.load("sprites/chunks/pfvz.png"),
    };
    commands.insert_resource(chunks);
}
