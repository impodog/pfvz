use crate::prelude::*;

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
