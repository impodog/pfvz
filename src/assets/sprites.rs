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

/// Load an animation from directory
fn load_animation(server: &Res<AssetServer>, base: &str, delta: Duration) -> Arc<sprite::FrameArr> {
    let mut i = 1;
    let mut frames = Vec::new();
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
    Arc::new(sprite::FrameArr { frames, delta })
}

#[derive(Resource)]
pub struct SpriteChunks {
    pub pvfz: Handle<Image>,
    pub background: Handle<Image>,
    pub slot: Handle<Image>,
    pub highlight: Handle<Image>,
    pub final_wave: Handle<Image>,
    pub alert: Handle<Image>,
    pub zayb: Handle<Image>,
    pub cooldown: Handle<Image>,
    pub you_win: Handle<Image>,
    pub white: Handle<Image>,
    pub shovel: Handle<Image>,
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
    pub peashooter_concept: Handle<Image>,
    pub sunflower: Arc<sprite::FrameArr>,
    pub cherry_bomb: Arc<sprite::FrameArr>,
    pub boom: Arc<sprite::FrameArr>,
    pub wall_nut: Arc<sprite::FrameArr>,
    pub wall_nut_damaged: Arc<sprite::FrameArr>,
    pub wall_nut_destroyed: Arc<sprite::FrameArr>,
    pub bowling_nut_concept: Handle<Image>,
    pub potato_mine: Arc<sprite::FrameArr>,
    pub potato_mine_preparing: Arc<sprite::FrameArr>,
    pub spudow: Arc<sprite::FrameArr>,
}

#[derive(Resource)]
pub struct SpriteZombies {
    pub basic: Arc<sprite::FrameArr>,
    pub basic_dying: Arc<sprite::FrameArr>,
    pub arm: Arc<sprite::FrameArr>,
    pub roadcone: Arc<sprite::FrameArr>,
    pub roadcone_broken: Arc<sprite::FrameArr>,
    pub roadcone_concept: Handle<Image>,
    pub bucket: Arc<sprite::FrameArr>,
    pub bucket_broken: Arc<sprite::FrameArr>,
    pub bucket_destroyed: Arc<sprite::FrameArr>,
    pub bucket_concept: Handle<Image>,
    pub flag: Arc<sprite::FrameArr>,
    pub flag_concept: Handle<Image>,
    pub all_star: Arc<sprite::FrameArr>,
    pub all_star_running: Arc<sprite::FrameArr>,
    pub all_star_dying: Arc<sprite::FrameArr>,
    pub helmet: Arc<sprite::FrameArr>,
    pub helmet_broken: Arc<sprite::FrameArr>,
    pub helmet_destroyed: Arc<sprite::FrameArr>,
    pub all_star_concept: Handle<Image>,
}

#[derive(Resource)]
pub struct SpriteItems {
    pub sun: Arc<sprite::FrameArr>,
}

fn init_chunks(mut commands: Commands, server: Res<AssetServer>) {
    let chunks = SpriteChunks {
        pvfz: server.load("sprites/chunks/pfvz.png"),
        background: server.load("sprites/chunks/background.png"),
        slot: server.load("sprites/chunks/slot.png"),
        highlight: server.load("sprites/chunks/highlight.png"),
        final_wave: server.load("sprites/chunks/final.png"),
        alert: server.load("sprites/chunks/alert.png"),
        zayb: server.load("sprites/chunks/zayb.png"),
        cooldown: server.load("sprites/chunks/cooldown.png"),
        you_win: server.load("sprites/chunks/you_win.png"),
        white: server.load("sprites/chunks/white.png"),
        shovel: server.load("sprites/chunks/shovel.png"),
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
        peashooter_concept: server.load("sprites/plants/peashooter/concept.png"),
        sunflower: load_animation(
            &server,
            "sprites/plants/sunflower",
            Duration::from_millis(300),
        ),
        cherry_bomb: load_animation(
            &server,
            "sprites/plants/cherry_bomb",
            Duration::from_millis(100),
        ),
        boom: load_animation(&server, "sprites/plants/boom", Duration::from_millis(100)),
        wall_nut: load_animation(
            &server,
            "sprites/plants/wall_nut",
            Duration::from_millis(500),
        ),
        wall_nut_damaged: load_animation(
            &server,
            "sprites/plants/wall_nut_damaged",
            Duration::from_millis(500),
        ),
        wall_nut_destroyed: load_animation(
            &server,
            "sprites/plants/wall_nut_destroyed",
            Duration::from_millis(500),
        ),
        bowling_nut_concept: server.load("sprites/plants/bowling_nut/concept.png"),
        potato_mine: load_animation(
            &server,
            "sprites/plants/potato_mine",
            Duration::from_millis(700),
        ),
        potato_mine_preparing: load_animation(
            &server,
            "sprites/plants/potato_mine_preparing",
            Duration::from_millis(800),
        ),
        spudow: load_animation(&server, "sprites/plants/spudow", Duration::from_millis(100)),
    };
    commands.insert_resource(plants);
}

fn init_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let zombies = SpriteZombies {
        basic: load_animation(&server, "sprites/zombies/basic", Duration::from_millis(400)),
        basic_dying: load_animation(
            &server,
            "sprites/zombies/basic_dying",
            Duration::from_millis(400),
        ),
        arm: load_animation(&server, "sprites/zombies/arm", Duration::from_millis(400)),
        roadcone: load_animation(
            &server,
            "sprites/zombies/roadcone",
            Duration::from_millis(200),
        ),
        roadcone_broken: load_animation(
            &server,
            "sprites/zombies/roadcone_broken",
            Duration::from_millis(200),
        ),
        roadcone_concept: server.load("sprites/zombies/roadcone/concept.png"),
        bucket: load_animation(
            &server,
            "sprites/zombies/bucket",
            Duration::from_millis(300),
        ),
        bucket_broken: load_animation(
            &server,
            "sprites/zombies/bucket_broken",
            Duration::from_millis(300),
        ),
        bucket_destroyed: load_animation(
            &server,
            "sprites/zombies/bucket_destroyed",
            Duration::from_millis(300),
        ),
        bucket_concept: server.load("sprites/zombies/bucket/concept.png"),
        flag: load_animation(&server, "sprites/zombies/flag", Duration::from_millis(400)),
        flag_concept: server.load("sprites/zombies/flag/concept.png"),
        all_star: load_animation(
            &server,
            "sprites/zombies/all_star",
            Duration::from_millis(600),
        ),
        all_star_running: load_animation(
            &server,
            "sprites/zombies/all_star_running",
            Duration::from_millis(100),
        ),
        all_star_dying: load_animation(
            &server,
            "sprites/zombies/all_star_dying",
            Duration::from_millis(300),
        ),
        helmet: load_animation(
            &server,
            "sprites/zombies/helmet",
            Duration::from_millis(400),
        ),
        helmet_broken: load_animation(
            &server,
            "sprites/zombies/helmet_broken",
            Duration::from_millis(400),
        ),
        helmet_destroyed: load_animation(
            &server,
            "sprites/zombies/helmet_destroyed",
            Duration::from_millis(400),
        ),
        all_star_concept: server.load("sprites/zombies/all_star/concept.png"),
    };
    commands.insert_resource(zombies);
}

fn init_items(mut commands: Commands, server: Res<AssetServer>) {
    let items = SpriteItems {
        sun: load_animation(&server, "sprites/items/sun", Duration::from_millis(100)),
    };
    commands.insert_resource(items);
}
