use crate::prelude::*;

pub(super) struct GameZombiePlugin;

impl Plugin for GameZombiePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZombieTypes>();
        app.add_event::<ZombieAction>();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zombie(pub Id);

#[derive(Component, Debug)]
pub struct ZombieRelevant;

#[derive(Debug, Clone)]
pub struct ZombieFun {
    pub spawn: SystemId,
    pub die: SystemId,
    pub damage: SystemId,
}

#[derive(Resource, Default, Debug)]
pub struct ZombieTypes(pub HashMap<Id, ZombieFun>);

#[derive(Event, Debug)]
pub enum ZombieAction {
    Spawn(Id, game::Position),
    Damage(Entity, u32),
}
