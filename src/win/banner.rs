use crate::prelude::*;

pub(super) struct WinBannerPlugin;

impl Plugin for WinBannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::GlobalStates::Win), (spawn_banner,));
    }
}

fn spawn_banner(mut commands: Commands, chunks: Res<assets::SpriteChunks>) {
    commands.spawn((
        level::Banner::new(Duration::from_millis(10000)),
        game::Position::new_xy(2.0, 0.0),
        SpriteBundle {
            texture: chunks.you_win.clone(),
            ..Default::default()
        },
    ));
}
