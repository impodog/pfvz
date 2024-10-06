use crate::prelude::*;

pub(super) struct CompnDefaultPlugin;

impl Plugin for CompnDefaultPlugin {
    fn build(&self, app: &mut App) {
        *system_do_nothing.write().unwrap() = Some(app.register_system(do_nothing));
        *system_spawn_not.write().unwrap() = Some(app.register_system(spawn_not));
        *system_die.write().unwrap() = Some(app.register_system(die));
        *system_die_not.write().unwrap() = Some(app.register_system(die_not));
        *system_damage.write().unwrap() = Some(app.register_system(damage));
    }
}

game_conf!(pub system system_do_nothing, Entity);
game_conf!(pub system system_spawn_not, game::LogicPosition);
game_conf!(pub system system_die, Entity);
game_conf!(pub system system_die_not, Entity);
game_conf!(pub system system_damage, (Entity, u32));

pub(crate) fn spawn_not(In(_logic): In<game::LogicPosition>) {}

pub(crate) fn die(
    In(entity): In<Entity>,
    mut commands: Commands,
    q_kill: Query<&game::Overlay, With<compn::NeverKillWhenActive>>,
) {
    if let Some(commands) = commands.get_entity(entity) {
        if !q_kill.get(entity).is_ok_and(|overlay| !overlay.is_zero()) {
            commands.despawn_recursive();
        }
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
