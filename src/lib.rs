pub mod config;
pub mod info;
mod prelude;

pub fn start_pfvz() {
    use prelude::*;
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, config::ConfigPlugin, info::InfoPlugin));
    app.run();
}
