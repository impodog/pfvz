use crate::prelude::*;

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

pub(super) fn init_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let zombies = SpriteZombies {
        basic: super::load_animation(&server, "sprites/zombies/basic", Duration::from_millis(400)),
        basic_dying: super::load_animation(
            &server,
            "sprites/zombies/basic_dying",
            Duration::from_millis(400),
        ),
        arm: super::load_animation(&server, "sprites/zombies/arm", Duration::from_millis(400)),
        roadcone: super::load_animation(
            &server,
            "sprites/zombies/roadcone",
            Duration::from_millis(200),
        ),
        roadcone_broken: super::load_animation(
            &server,
            "sprites/zombies/roadcone_broken",
            Duration::from_millis(200),
        ),
        roadcone_concept: server.load("sprites/zombies/roadcone/concept.png"),
        bucket: super::load_animation(
            &server,
            "sprites/zombies/bucket",
            Duration::from_millis(300),
        ),
        bucket_broken: super::load_animation(
            &server,
            "sprites/zombies/bucket_broken",
            Duration::from_millis(300),
        ),
        bucket_destroyed: super::load_animation(
            &server,
            "sprites/zombies/bucket_destroyed",
            Duration::from_millis(300),
        ),
        bucket_concept: server.load("sprites/zombies/bucket/concept.png"),
        flag: super::load_animation(&server, "sprites/zombies/flag", Duration::from_millis(400)),
        flag_concept: server.load("sprites/zombies/flag/concept.png"),
        all_star: super::load_animation(
            &server,
            "sprites/zombies/all_star",
            Duration::from_millis(600),
        ),
        all_star_running: super::load_animation(
            &server,
            "sprites/zombies/all_star_running",
            Duration::from_millis(100),
        ),
        all_star_dying: super::load_animation(
            &server,
            "sprites/zombies/all_star_dying",
            Duration::from_millis(300),
        ),
        helmet: super::load_animation(
            &server,
            "sprites/zombies/helmet",
            Duration::from_millis(400),
        ),
        helmet_broken: super::load_animation(
            &server,
            "sprites/zombies/helmet_broken",
            Duration::from_millis(400),
        ),
        helmet_destroyed: super::load_animation(
            &server,
            "sprites/zombies/helmet_destroyed",
            Duration::from_millis(400),
        ),
        all_star_concept: server.load("sprites/zombies/all_star/concept.png"),
    };
    commands.insert_resource(zombies);
}
