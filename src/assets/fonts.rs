use crate::prelude::*;

pub(super) struct AssetsFontsPlugin;

impl Plugin for AssetsFontsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_fonts,));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DefaultFont(pub Handle<Font>);

fn init_fonts(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(DefaultFont(server.load("fonts/zed.ttf")));
}
