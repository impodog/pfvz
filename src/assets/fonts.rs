use crate::prelude::*;

pub(super) struct AssetsFontsPlugin;

impl Plugin for AssetsFontsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (init_fonts,));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DefaultFont(pub Handle<Font>);

#[derive(Resource, Debug, Clone)]
pub struct ItalicsFont(pub Handle<Font>);

#[derive(Resource, Debug, Clone)]
pub struct ChunksFont(pub Handle<Font>);

fn init_fonts(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(DefaultFont(server.load("fonts/zed.ttf")));
    commands.insert_resource(ItalicsFont(server.load("fonts/cas-italics.ttf")));
    commands.insert_resource(ChunksFont(server.load("fonts/fira.ttf")));
}
