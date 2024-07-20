use super::*;
use crate::prelude::*;

pub struct CompnPlugin;

impl Plugin for CompnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shooter::ShooterPlugin,
            proj::CompnProjPlugin,
            walker::WalkerPlugin,
        ));
    }
}
