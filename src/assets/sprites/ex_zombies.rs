use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteExZombies {
    pub rally_flag: Arc<sprite::FrameArr>,
    pub rally_flag_damaged: Arc<sprite::FrameArr>,
    pub rally_zombie_concept: Handle<Image>,
    pub brick: Arc<sprite::FrameArr>,
    pub brick_damaged: Arc<sprite::FrameArr>,
    pub brick_broken: Arc<sprite::FrameArr>,
    pub brick_destroyed: Arc<sprite::FrameArr>,
    pub brick_zombie_concept: Handle<Image>,
    pub giga_all_star: Arc<sprite::FrameArr>,
    pub giga_all_star_running: Arc<sprite::FrameArr>,
    pub giga_all_star_dying: Arc<sprite::FrameArr>,
    pub giga_all_star_sliding: Arc<sprite::FrameArr>,
    pub giga_helmet: Arc<sprite::FrameArr>,
    pub giga_helmet_damaged: Arc<sprite::FrameArr>,
    pub giga_helmet_destroyed: Arc<sprite::FrameArr>,
    pub giga_all_star_concept: Handle<Image>,
    pub sunday_edition: Arc<sprite::FrameArr>,
    pub sunday_edition_damaged: Arc<sprite::FrameArr>,
    pub sunday_edition_destroyed: Arc<sprite::FrameArr>,
    pub sunday_edition_zombie: Arc<sprite::FrameArr>,
    pub sunday_edition_zombie_dying: Arc<sprite::FrameArr>,
    pub mirror: Arc<sprite::FrameArr>,
    pub mirror_damaged: Arc<sprite::FrameArr>,
    pub mirror_zombie_concept: Handle<Image>,
    pub foodcan: Arc<sprite::FrameArr>,
    pub foodcan_damaged: Arc<sprite::FrameArr>,
    pub foodcan_zombie: Arc<sprite::FrameArr>,
    pub foodcan_zombie_dying: Arc<sprite::FrameArr>,
}

pub(super) fn init_ex_zombies(mut commands: Commands, server: Res<AssetServer>) {
    let ex_zombies = SpriteExZombies {
        rally_flag: super::load_animation(
            &server,
            "sprites/zombies/rally_flag",
            Duration::from_millis(150),
        ),
        rally_flag_damaged: super::load_animation(
            &server,
            "sprites/zombies/rally_flag_damaged",
            Duration::from_millis(175),
        ),
        rally_zombie_concept: server.load("sprites/zombies/rally_flag/concept.png"),
        brick: super::load_animation(&server, "sprites/zombies/brick", Duration::from_millis(150)),
        brick_damaged: super::load_animation(
            &server,
            "sprites/zombies/brick_damaged",
            Duration::from_millis(150),
        ),
        brick_broken: super::load_animation(
            &server,
            "sprites/zombies/brick_broken",
            Duration::from_millis(150),
        ),
        brick_destroyed: super::load_animation(
            &server,
            "sprites/zombies/brick_destroyed",
            Duration::from_millis(150),
        ),
        brick_zombie_concept: server.load("sprites/zombies/brick/concept.png"),
        giga_all_star: super::load_animation(
            &server,
            "sprites/zombies/giga_all_star",
            Duration::from_millis(500),
        ),
        giga_all_star_running: super::load_animation(
            &server,
            "sprites/zombies/giga_all_star_running",
            Duration::from_millis(100),
        ),
        giga_all_star_dying: super::load_animation(
            &server,
            "sprites/zombies/giga_all_star_dying",
            Duration::from_millis(300),
        ),
        giga_all_star_sliding: super::load_animation(
            &server,
            "sprites/zombies/giga_all_star_sliding",
            Duration::from_millis(300),
        ),
        giga_helmet: super::load_animation(
            &server,
            "sprites/zombies/giga_helmet",
            Duration::from_millis(400),
        ),
        giga_helmet_damaged: super::load_animation(
            &server,
            "sprites/zombies/giga_helmet_damaged",
            Duration::from_millis(400),
        ),
        giga_helmet_destroyed: super::load_animation(
            &server,
            "sprites/zombies/giga_helmet_destroyed",
            Duration::from_millis(400),
        ),
        giga_all_star_concept: server.load("sprites/zombies/giga_all_star/concept.png"),
        sunday_edition: super::load_animation(
            &server,
            "sprites/zombies/sunday_edition",
            Duration::from_millis(150),
        ),
        sunday_edition_damaged: super::load_animation(
            &server,
            "sprites/zombies/sunday_edition_damaged",
            Duration::from_millis(150),
        ),
        sunday_edition_destroyed: super::load_animation(
            &server,
            "sprites/zombies/sunday_edition_destroyed",
            Duration::from_millis(150),
        ),
        sunday_edition_zombie: super::load_animation(
            &server,
            "sprites/zombies/sunday_edition_zombie",
            Duration::from_millis(350),
        ),
        sunday_edition_zombie_dying: super::load_animation(
            &server,
            "sprites/zombies/sunday_edition_zombie_dying",
            Duration::from_millis(375),
        ),
        mirror: super::load_animation(&server, "sprites/zombies/mirror", Duration::from_millis(33)),
        mirror_damaged: super::load_animation(
            &server,
            "sprites/zombies/mirror_damaged",
            Duration::from_millis(50),
        ),
        mirror_zombie_concept: server.load("sprites/zombies/mirror/concept.png"),
        foodcan: super::load_animation(
            &server,
            "sprites/zombies/foodcan",
            Duration::from_millis(150),
        ),
        foodcan_damaged: super::load_animation(
            &server,
            "sprites/zombies/foodcan_damaged",
            Duration::from_millis(175),
        ),
        foodcan_zombie: super::load_animation(
            &server,
            "sprites/zombies/foodcan_zombie",
            Duration::from_millis(250),
        ),
        foodcan_zombie_dying: super::load_animation(
            &server,
            "sprites/zombies/foodcan_zombie",
            Duration::from_millis(300),
        ),
    };
    commands.insert_resource(ex_zombies);
}
