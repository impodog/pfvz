use crate::prelude::*;

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
    pub snow_pea: Arc<sprite::FrameArr>,
    pub snow: Arc<sprite::FrameArr>,
    pub iceberg_lettuce: Arc<sprite::FrameArr>,
    pub repeater: Arc<sprite::FrameArr>,
    pub puff_shroom: Arc<sprite::FrameArr>,
    pub spore: Arc<sprite::FrameArr>,
    pub grave1: Arc<sprite::FrameArr>,
    pub grave2: Arc<sprite::FrameArr>,
    pub sun_shroom_small: Arc<sprite::FrameArr>,
    pub sun_shroom_big: Arc<sprite::FrameArr>,
    pub grave_buster: Arc<sprite::FrameArr>,
    pub fume_shroom: Arc<sprite::FrameArr>,
    pub fume_shroom_shoot: Arc<sprite::FrameArr>,
    pub fume: Arc<sprite::FrameArr>,
    pub scaredy_shroom: Arc<sprite::FrameArr>,
}

pub(super) fn init_plants(mut commands: Commands, server: Res<AssetServer>) {
    let plants = SpritePlants {
        pea: super::load_animation(&server, "sprites/plants/pea", Duration::from_millis(50)),
        peashooter: super::load_animation(
            &server,
            "sprites/plants/peashooter",
            Duration::from_millis(100),
        ),
        peashooter_concept: server.load("sprites/plants/peashooter/concept.png"),
        sunflower: super::load_animation(
            &server,
            "sprites/plants/sunflower",
            Duration::from_millis(300),
        ),
        cherry_bomb: super::load_animation(
            &server,
            "sprites/plants/cherry_bomb",
            Duration::from_millis(100),
        ),
        boom: super::load_animation(&server, "sprites/plants/boom", Duration::from_millis(100)),
        wall_nut: super::load_animation(
            &server,
            "sprites/plants/wall_nut",
            Duration::from_millis(500),
        ),
        wall_nut_damaged: super::load_animation(
            &server,
            "sprites/plants/wall_nut_damaged",
            Duration::from_millis(500),
        ),
        wall_nut_destroyed: super::load_animation(
            &server,
            "sprites/plants/wall_nut_destroyed",
            Duration::from_millis(500),
        ),
        bowling_nut_concept: server.load("sprites/plants/bowling_nut/concept.png"),
        potato_mine: super::load_animation(
            &server,
            "sprites/plants/potato_mine",
            Duration::from_millis(700),
        ),
        potato_mine_preparing: super::load_animation(
            &server,
            "sprites/plants/potato_mine_preparing",
            Duration::from_millis(800),
        ),
        spudow: super::load_animation(&server, "sprites/plants/spudow", Duration::from_millis(100)),
        snow_pea: super::load_animation(
            &server,
            "sprites/plants/snow_pea",
            Duration::from_millis(200),
        ),
        snow: super::load_animation(&server, "sprites/plants/snow", Duration::from_millis(50)),
        repeater: super::load_animation(
            &server,
            "sprites/plants/repeater",
            Duration::from_millis(150),
        ),
        iceberg_lettuce: super::load_animation(
            &server,
            "sprites/plants/iceberg_lettuce",
            Duration::from_millis(200),
        ),
        puff_shroom: super::load_animation(
            &server,
            "sprites/plants/puff_shroom",
            Duration::from_millis(300),
        ),
        spore: super::load_animation(&server, "sprites/plants/spore", Duration::from_millis(100)),
        grave1: super::load_animation(&server, "sprites/plants/grave1", Duration::from_millis(100)),
        grave2: super::load_animation(&server, "sprites/plants/grave2", Duration::from_millis(100)),
        sun_shroom_small: super::load_animation(
            &server,
            "sprites/plants/sun_shroom_small",
            Duration::from_millis(250),
        ),
        sun_shroom_big: super::load_animation(
            &server,
            "sprites/plants/sun_shroom_big",
            Duration::from_millis(350),
        ),
        grave_buster: super::load_animation(
            &server,
            "sprites/plants/grave_buster",
            Duration::from_millis(100),
        ),
        fume_shroom: super::load_animation(
            &server,
            "sprites/plants/fume_shroom",
            Duration::from_millis(300),
        ),
        fume_shroom_shoot: super::load_animation(
            &server,
            "sprites/plants/fume_shroom_shoot",
            Duration::from_millis(150),
        ),
        fume: super::load_animation(&server, "sprites/plants/fume", Duration::from_millis(100)),
        scaredy_shroom: super::load_animation(
            &server,
            "sprites/plants/scaredy_shroom",
            Duration::from_millis(300),
        ),
    };
    commands.insert_resource(plants);
}
