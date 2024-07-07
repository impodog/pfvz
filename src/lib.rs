pub mod assets;
pub mod config;
pub mod game;
pub mod info;
pub mod level;
mod prelude;
pub mod sprite;

pub fn start_pfvz() {
    use prelude::*;
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        assets::AssetsPlugin,
        config::ConfigPlugin,
        game::GamePlugin,
        info::InfoPlugin,
        sprite::SpritePlugin,
        level::LevelPlugin,
    ));
    app.run();
}
