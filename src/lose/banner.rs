use crate::prelude::*;

pub(super) struct LoseBannerPlugin;

impl Plugin for LoseBannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Lose), (spawn_banner,));
    }
}

fn spawn_banner(mut commands: Commands, chunks: Res<assets::SpriteChunks>) {
    commands.spawn((
        level::Banner::new(Duration::from_millis(10000)),
        game::Position::new_xy(0.0, 0.0),
        SpriteBundle {
            texture: chunks.zayb.clone(),
            ..Default::default()
        },
    ));
}
