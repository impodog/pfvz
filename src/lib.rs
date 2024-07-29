pub mod assets;
pub mod choose;
pub mod collectible;
pub mod compn;
pub mod config;
pub mod game;
pub mod info;
pub mod level;
pub mod lose;
pub mod plants;
mod prelude;
pub mod save;
pub mod sprite;
pub mod win;
pub mod zombies;

pub fn start_pfvz() {
    use prelude::*;
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        bevy_framepace::FramepacePlugin,
        assets::AssetsPlugin,
        collectible::CollectiblePlugin,
        config::ConfigPlugin,
        game::GamePlugin,
        info::InfoPlugin,
        sprite::SpritePlugin,
        level::LevelPlugin,
        save::SavePlugin,
        plants::PlantsPlugin,
        zombies::ZombiesPlugin,
        compn::CompnPlugin,
        lose::LosePlugin,
        win::WinPlugin,
    ));
    app.add_plugins((choose::ChoosePlugin,));
    app.run();
}
