use crate::prelude::*;

#[derive(Resource)]
pub struct SpriteExPlants {
    pub twin_sunflower: Arc<sprite::FrameArr>,
}

pub(super) fn init_ex_plants(mut commands: Commands, server: Res<AssetServer>) {
    let ex_plants = SpriteExPlants {
        twin_sunflower: super::load_animation(
            &server,
            "sprites/plants/twin_sunflower",
            Duration::from_millis(325),
        ),
    };
    commands.insert_resource(ex_plants);
}
