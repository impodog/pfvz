use crate::prelude::*;

pub(super) struct GamePlantPlugin;

impl Plugin for GamePlantPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantLayout>();
        app.add_systems(PostUpdate, (scan_plants,));
        app.add_systems(OnEnter(info::GlobalStates::Play), (renew_layout,));
    }
}

#[derive(Component)]
pub struct Plant;

#[derive(Component)]
pub struct PlantRelevant;

#[derive(Resource, Default, Debug)]
pub struct PlantLayout {
    pub plants: Vec<RwLock<Vec<Entity>>>,
}
impl PlantLayout {
    pub fn clear(&mut self, size: usize) {
        self.plants.clear();
        for _ in 0..size {
            self.plants.push(RwLock::new(Vec::new()));
        }
    }
}

fn renew_layout(mut plants: ResMut<PlantLayout>, level: Res<level::Level>) {
    let size = level.config.layout.size();
    plants.clear(size.0 * size.1);
}

fn scan_plants(plants: Res<PlantLayout>, q_plant: Query<&Plant>) {
    // NOTE: This scans every tile. May be a performance bottleneck!
    let mut remove = Vec::new();
    for lane in plants.plants.iter() {
        for (i, plant) in lane.read().unwrap().iter().enumerate() {
            if q_plant.get(*plant).is_err() {
                remove.push(i);
            }
        }
        // If one or more plants are missing, remove them from the layout
        if !remove.is_empty() {
            let mut lane = lane.write().unwrap();
            for i in remove.drain(..).rev() {
                lane.swap_remove(i);
            }
        }
    }
}
