use crate::prelude::*;

pub(super) struct GamePlantPlugin;

impl Plugin for GamePlantPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantLayout>();
        app.add_systems(PostUpdate, (scan_plants, add_plants));
        app.add_systems(OnEnter(info::GlobalStates::Play), (renew_layout,));
    }
}

#[derive(Component)]
pub struct Plant;

#[derive(Component)]
pub struct PlantRelevant;

// Indicates a grave, or hole
#[derive(Component)]
pub struct NotPlanted;

#[derive(Resource, Default, Debug)]
pub struct PlantLayout {
    pub plants: Vec<RwLock<Vec<Entity>>>,
    in_layout: RwLock<BTreeSet<Entity>>,
}
impl PlantLayout {
    pub fn clear(&mut self, size: usize) {
        self.plants.clear();
        for _ in 0..size {
            self.plants.push(RwLock::new(Vec::new()));
        }
    }

    pub fn top(&self, pos: usize) -> Option<Entity> {
        self.plants
            .get(pos)
            .and_then(|list| list.read().unwrap().last().cloned())
    }
}

fn renew_layout(mut plants: ResMut<PlantLayout>, level: Res<level::Level>) {
    let size = level.config.layout.size();
    plants.clear(size.0 * size.1);
}

fn scan_plants(plants: Res<PlantLayout>, q_plant: Query<&game::Position, With<Plant>>) {
    // NOTE: This scans every tile. May be a performance bottleneck!
    let mut remove = Vec::new();
    for lane in plants.plants.iter() {
        for (i, plant) in lane.read().unwrap().iter().enumerate() {
            if let Ok(_pos) = q_plant.get(*plant) {
            } else {
                remove.push(i);
            }
        }
        // If one or more plants are missing, remove them from the layout
        if !remove.is_empty() {
            let mut lane = lane.write().unwrap();
            for i in remove.drain(..).rev() {
                let entity = lane.swap_remove(i);
                plants.in_layout.write().unwrap().remove(&entity);
            }
        }
    }
}
fn add_plants(
    plants: Res<PlantLayout>,
    q_plant: Query<(Entity, &game::Position), Added<Plant>>,
    level: Res<level::Level>,
) {
    q_plant.par_iter().for_each(|(entity, pos)| {
        if !plants.in_layout.read().unwrap().contains(&entity) {
            plants.in_layout.write().unwrap().insert(entity);
            let i = level.config.layout.position_to_index(pos);
            if let Some(plants) = plants.plants.get(i) {
                plants.write().unwrap().push(entity);
            } else {
                error!(
                    "Plant at {:?} is outside the bounds and will not be monitored",
                    pos
                );
            }
        }
    });
}
