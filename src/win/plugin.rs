use super::*;
use crate::prelude::*;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((banner::WinBannerPlugin,));
    }
}
