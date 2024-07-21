use crate::prelude::*;

pub(super) struct AssetsSpritesPlugin;

impl Plugin for AssetsSpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (init_chunks, init_layouts, init_plants, init_zombies),
        );
    }
}

/// Load an animation from directory
fn load_animation(server: &Res<AssetServer>, base: &str, delta: Duration) -> Arc<sprite::FrameArr> {
    let mut i = 1;
    let mut frames = Vec::new();
    loop {
        let path = format!("{}/{}.png", base, i);
        if std::fs::exists(format!("assets/{}", path)).unwrap_or(false) {
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
    Arc::new(sprite::FrameArr { frames, delta })
}

#[derive(Resource)]
pub struct SpriteChunks {
    pub pvfz: Handle<Image>,
    pub background: Handle<Image>,
    pub slot: Handle<Image>,
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

#[derive(Resource)]
pub struct SpritePlants {
    pub pea: Arc<sprite::FrameArr>,
    pub peashooter: Arc<sprite::FrameArr>,
}

#[derive(Resource)]
pub struct SpriteZombies {
    pub basic: Arc<sprite::FrameArr>,
    pub arm: Arc<sprite::FrameArr>,
}

fn init_chunks(mut commands: Commands, server: Res<AssetServer>) {
    let chunks = SpriteChunks {
        pvfz: server.load("sprites/chunks/pfvz.png"),
        background: server.load("sprites/chunks/background.png"),
        slot: server.load("sprites/chunks/slot.png"),
    };
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(LOGICAL_WIDTH, LOGICAL_HEIGHT)),
            color: Color::LinearRgba(LinearRgba::new(1.0, 1.0, 1.0, 0.02)),
            ..Default::default()
        },
        texture: chunks.background.clone(),
        ..Default::default()
    });
    commands.insert_resource(chunks);
}

fn init_layouts(mut commands: Commands, server: Res<AssetServer>) {
    let layouts = SpriteLayouts {
        day: SpriteLayout::load(&server, "sprites/layouts/day"),
    };
    commands.insert_resource(layouts);
}

fn init_plants(mut commands: Commands, server: Res<AssetServer>) {
    let plants = SpritePlants {
        pea: load_animation(&server, "sprites/plants/pea", Duration::from_millis(50)),
        peashooter: load_animation(
            &server,
            "sprites/plants/peashooter",
            Duration::from_millis(100),
        ),
    };
    commands.insert_resource(plants);
}

fn init_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let zombies = SpriteZombies {
        basic: load_animation(&server, "sprites/zombies/basic", Duration::from_millis(200)),
        arm: load_animation(&server, "sprites/zombies/arm", Duration::from_millis(200)),
    };
    commands.insert_resource(zombies);
}
