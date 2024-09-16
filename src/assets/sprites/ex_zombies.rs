use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteExZombies {
    pub rally_flag: Arc<sprite::FrameArr>,
    pub rally_flag_damaged: Arc<sprite::FrameArr>,
    pub rally_zombie_concept: Handle<Image>,
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
    };
    commands.insert_resource(ex_zombies);
}
