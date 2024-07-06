use crate::prelude::*;

pub(super) struct GamePlantPlugin;

impl Plugin for GamePlantPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlantTypes>();
        app.add_event::<PlantAction>();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plant(pub Id);

#[derive(Component)]
pub struct PlantRelevant;

#[derive(Debug, Clone)]
pub struct PlantFun {
    pub spawn: SystemId,
    pub die: SystemId,
    pub damage: SystemId,
}

#[derive(Resource, Default, Debug)]
pub struct PlantTypes(pub HashMap<Id, PlantFun>);

#[derive(Event, Debug, Clone)]
pub enum PlantAction {
    Spawn(Id, game::Position),
    Damage(Entity, u32),
}
