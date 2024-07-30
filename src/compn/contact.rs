use crate::prelude::*;

pub(super) struct CompnContactPlugin;

impl Plugin for CompnContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (test_contact,));
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Contact {
    pub system: SystemId<(Entity, Entity)>,
}

fn test_contact(
    mut commands: Commands,
    q_contact: Query<(Entity, &Contact)>,
    collision: Res<game::Collision>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_plant: Query<(), With<game::Plant>>,
) {
    let work = RwLock::new(Vec::new());
    q_contact.par_iter().for_each(|(entity, contact)| {
        if let Some(coll) = collision.get(&entity) {
            let enemy = if q_zombie.get(entity).is_ok() {
                coll.iter().find_map(|plant| {
                    if q_plant.get(*plant).is_ok() {
                        Some(*plant)
                    } else {
                        None
                    }
                })
            } else {
                coll.iter().find_map(|zombie| {
                    if q_zombie.get(*zombie).is_ok() {
                        Some(*zombie)
                    } else {
                        None
                    }
                })
            };
            if let Some(enemy) = enemy {
                work.write()
                    .unwrap()
                    .push((contact.system, (entity, enemy)));
            }
        }
    });
    for (system, target) in RwLock::into_inner(work).unwrap().into_iter() {
        commands.run_system_with_input(system, target);
    }
}
