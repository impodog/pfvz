use crate::prelude::*;

pub(super) struct CompnDefaultPlugin;

impl Plugin for CompnDefaultPlugin {
    fn build(&self, app: &mut App) {
        *system_do_nothing.write().unwrap() = Some(app.register_system(do_nothing));
    }
}

game_conf!(pub system system_do_nothing, Entity);

pub(crate) fn die(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
}

pub(crate) fn die_not(In(_entity): In<Entity>) {
    // Do nothing on death
}

pub(crate) fn do_nothing(In(_): In<Entity>) {}

pub(crate) fn damage(
    In((entity, damage)): In<(Entity, u32)>,
    mut q_health: Query<&mut game::Health>,
) {
    if let Ok(mut health) = q_health.get_mut(entity) {
        health.decr(damage);
    }
}
