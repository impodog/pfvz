use crate::prelude::*;

pub(crate) fn die(In(entity): In<Entity>, mut commands: Commands) {
    commands.entity(entity).despawn_recursive();
}

pub(crate) fn damage(
    In((entity, damage)): In<(Entity, u32)>,
    mut q_health: Query<&mut game::Health>,
) {
    if let Ok(mut health) = q_health.get_mut(entity) {
        health.decr(damage);
    }
}
