use super::*;
use crate::prelude::*;

pub struct CompnPlugin;

impl Plugin for CompnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            default::CompnDefaultPlugin,
            shooter::CompnShooterPlugin,
            producer::CompnProducerPlugin,
            proj::CompnProjPlugin,
            walker::CompnWalkerPlugin,
            dying::CompnDyingPlugin,
            breaks::CompnBreaksPlugin,
            explode::CompnExplodePlugin,
            snow::CompnSnowPlugin,
            bowling::CompnBowlingPlugin,
            dog::CompnDogPlugin,
            contact::CompnContactPlugin,
            anim_do::CompnAnimDoPlugin,
        ));
    }
}
