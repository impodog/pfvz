use crate::prelude::*;

pub(super) struct CompnContactPlugin;

impl Plugin for CompnContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_contact_many_impl, test_contact, test_contact_many).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Contact {
    pub system: SystemId<(Entity, Entity)>,
}
pub type EntityList = SmallVec<[Entity; 3]>;

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

#[derive(Component, Debug, Clone, Copy)]
pub struct ContactMany {
    pub system: SystemId<(Entity, EntityList)>,
    pub interval: Duration,
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct ContactManyImpl(pub Timer);

fn add_contact_many_impl(
    mut commands: Commands,
    q_contact: Query<(Entity, &ContactMany), Added<ContactMany>>,
) {
    q_contact.iter().for_each(|(entity, contact)| {
        commands
            .entity(entity)
            .try_insert(ContactManyImpl(Timer::new(
                contact.interval,
                TimerMode::Repeating,
            )));
    });
}

fn test_contact_many(
    mut commands: Commands,
    mut q_contact: Query<(Entity, &game::Overlay, &ContactMany, &mut ContactManyImpl)>,
    collision: Res<game::Collision>,
    q_zombie: Query<(), With<game::Zombie>>,
    q_plant: Query<(), With<game::Plant>>,
) {
    let work = RwLock::new(Vec::new());
    q_contact
        .par_iter_mut()
        .for_each(|(entity, overlay, contact, mut contact_impl)| {
            contact_impl.tick(overlay.delta());
            if contact_impl.just_finished() {
                if let Some(coll) = collision.get(&entity) {
                    let enemies = coll.iter().filter_map(|&enemy| {
                        if q_zombie.get(enemy).is_ok() {
                            if q_plant.get(entity).is_ok() {
                                Some(enemy)
                            } else {
                                None
                            }
                        } else if q_plant.get(enemy).is_ok() {
                            if q_zombie.get(entity).is_ok() {
                                Some(enemy)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    });
                    let enemies: SmallVec<_> = enemies.collect();
                    if !enemies.is_empty() {
                        work.write()
                            .unwrap()
                            .push((contact.system, (entity, enemies)));
                    }
                }
            }
        });
    for (system, target) in RwLock::into_inner(work).unwrap().into_iter() {
        commands.run_system_with_input(system, target);
    }
}
