use super::*;
use crate::prelude::*;

pub struct CompnPlugin;

impl Plugin for CompnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            shooter::CompnShooterPlugin,
            proj::CompnProjPlugin,
            walker::CompnWalkerPlugin,
            dying::CompnDyingPlugin,
        ));
    }
}
