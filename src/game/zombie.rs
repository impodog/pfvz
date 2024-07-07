use crate::prelude::*;

pub(super) struct GameZombiePlugin;

impl Plugin for GameZombiePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zombie;

#[derive(Component, Debug)]
pub struct ZombieRelevant;
