use super::*;
use crate::prelude::*;

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            factors::PlantsFactorsPlugin,
            planter::PlantsPlanterPlugin,
            peas::PlantsPeaPlugin,
            producers::PlantsProducersPlugin,
            explode::PlantsExplodePlugin,
            defense::PlantsDefensePlugin,
            bowling::PlantsBowlingPlugin,
            contact::PlantsContactPlugin,
            special::PlantsSpecialPlugin,
            spores::PlantsSporesPlugin,
            buster::PlantsBusterPlugin,
            fume::PlantsFumePlugin,
            instant::PlantsInstantPlugin,
            pads::PlantsPadsPlugin,
            squash::PlantsSquashPlugin,
        ));
        app.add_plugins((
            spike::PlantsSpikePlugin,
            fire::PlantsFirePlugin,
            bonk_choy::PlantsBonkChoyPlugin,
            plantern::PlantsPlanternPlugin,
            hurrikale::PlantsHurrikalePlugin,
            starfruit::PlantsStarfruitPlugin,
            magnet::PlantsMagnetPlugin,
            blover::PlantsBloverPlugin,
            cactus::PlantsCactusPlugin,
        ));
    }
}
