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
    pub newspaper_zombie: Arc<sprite::FrameArr>,
    pub newspaper_dying: Arc<sprite::FrameArr>,
    pub newspaper: Arc<sprite::FrameArr>,
    pub newspaper_broken: Arc<sprite::FrameArr>,
    pub screen_door: Arc<sprite::FrameArr>,
    pub screen_door_broken: Arc<sprite::FrameArr>,
    pub screen_door_destroyed: Arc<sprite::FrameArr>,
    pub screen_door_concept: Handle<Image>,
    pub trashcan_zombie: Arc<sprite::FrameArr>,
    pub trashcan_zombie_dying: Arc<sprite::FrameArr>,
    pub trashcan: Arc<sprite::FrameArr>,
    pub trashcan_broken: Arc<sprite::FrameArr>,
    pub hidden_zombie: Arc<sprite::FrameArr>,
    pub tube: Arc<sprite::FrameArr>,
    pub snorkel_zombie: Arc<sprite::FrameArr>,
    pub snorkel_zombie_dying: Arc<sprite::FrameArr>,
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
        newspaper_zombie: super::load_animation(
            &server,
            "sprites/zombies/newspaper_zombie",
            Duration::from_millis(400),
        ),
        newspaper_dying: super::load_animation(
            &server,
            "sprites/zombies/newspaper_dying",
            Duration::from_millis(400),
        ),
        newspaper: super::load_animation(
            &server,
            "sprites/zombies/newspaper",
            Duration::from_millis(200),
        ),
        newspaper_broken: super::load_animation(
            &server,
            "sprites/zombies/newspaper_broken",
            Duration::from_millis(250),
        ),
        screen_door: super::load_animation(
            &server,
            "sprites/zombies/screen_door",
            Duration::from_millis(300),
        ),
        screen_door_broken: super::load_animation(
            &server,
            "sprites/zombies/screen_door_broken",
            Duration::from_millis(300),
        ),
        screen_door_destroyed: super::load_animation(
            &server,
            "sprites/zombies/screen_door_destroyed",
            Duration::from_millis(300),
        ),
        screen_door_concept: server.load("sprites/zombies/screen_door/concept.png"),
        trashcan_zombie: super::load_animation(
            &server,
            "sprites/zombies/trashcan_zombie",
            Duration::from_millis(300),
        ),
        trashcan_zombie_dying: super::load_animation(
            &server,
            "sprites/zombies/trashcan_zombie_dying",
            Duration::from_millis(300),
        ),
        trashcan: super::load_animation(
            &server,
            "sprites/zombies/trashcan",
            Duration::from_millis(300),
        ),
        trashcan_broken: super::load_animation(
            &server,
            "sprites/zombies/trashcan_broken",
            Duration::from_millis(300),
        ),
        hidden_zombie: super::load_animation(
            &server,
            "sprites/zombies/hidden",
            Duration::from_millis(300),
        ),
        tube: super::load_animation(&server, "sprites/zombies/tube", Duration::from_millis(300)),
        snorkel_zombie: super::load_animation(
            &server,
            "sprites/zombies/snorkel_zombie",
            Duration::from_millis(300),
        ),
        snorkel_zombie_dying: super::load_animation(
            &server,
            "sprites/zombies/snorkel_zombie_dying",
            Duration::from_millis(300),
        ),
    };
    commands.insert_resource(zombies);
}
