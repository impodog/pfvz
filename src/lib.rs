#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod ach;
pub mod almanac;
pub mod assets;
pub mod choose;
pub mod collectible;
pub mod compn;
pub mod config;
mod dave;
pub mod ex_plants;
pub mod ex_zombies;
pub mod game;
pub mod info;
pub mod level;
pub mod lose;
pub mod menu;
pub mod modes;
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
        bevy_kira_audio::AudioPlugin,
        bevy_framepace::FramepacePlugin,
        bevy_egui::EguiPlugin,
        assets::AssetsPlugin,
        collectible::CollectiblePlugin,
        config::ConfigPlugin,
        game::GamePlugin,
        info::InfoPlugin,
        sprite::SpritePlugin,
        level::LevelPlugin,
        save::SavePlugin,
        compn::CompnPlugin,
        plants::PlantsPlugin,
        zombies::ZombiesPlugin,
    ));
    app.add_plugins((
        ex_zombies::ExZombiesPlugin,
        ex_plants::ExPlantsPlugin,
        choose::ChoosePlugin,
        dave::DavePlugin,
        modes::ModesPlugin,
        lose::LosePlugin,
        win::WinPlugin,
        menu::MenuPlugin,
        ach::AchPlugin,
        almanac::AlmanacPlugin,
    ));
    app.run();
}
