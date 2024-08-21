use crate::prelude::*;

pub(super) struct PlantsMagnetPlugin;

impl Plugin for PlantsMagnetPlugin {
    fn build(&self, app: &mut App) {
        initialize(&magnet_shroom_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (magnet_work,));
        *magnet_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_magnet_shroom),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems magnet_shroom_systems);

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct MagnetShroomCount(pub usize);

fn spawn_magnet_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&MAGNET_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.magnet_shroom.clone()),
        creature.hitbox,
        MagnetShroomCount::default(),
        game::Health::from(factors.magnet_shroom.health),
        SpriteBundle::default(),
    ));
}

/// The entity and its distance to the magnet-shroom
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MagnetEntity(Orderedf32, Entity);

fn magnet_work(
    commands: ParallelCommands,
    action: EventWriter<game::CreatureAction>,
    mut q_magnet: Query<(
        Entity,
        &game::Overlay,
        &game::Position,
        &mut MagnetShroomCount,
    )>,
    factors: Res<plants::PlantFactors>,
    q_magnetic: Query<
        (Entity, Option<&Parent>, &game::Position, &game::HitBox),
        With<game::Magnetic>,
    >,
    q_pos: Query<&game::Position>,
    q_armor: Query<(&Parent, &Handle<Image>, &game::HitBox), With<game::Armor>>,
    q_digger: Query<(), With<zombies::DiggerStatus>>,
    digger_go_up: ResMut<zombies::DiggerPleaseGoUp>,
) {
    let action = Mutex::new(action);
    let digger_go_up = Mutex::new(digger_go_up);
    q_magnet
        .par_iter_mut()
        .for_each(|(entity, overlay, pos, mut count)| {
            if overlay.factor() == 0.0 {
                return;
            }

            let diff = factors.magnet_shroom.objects.saturating_sub(count.0);
            if diff == 0 {
                action
                    .lock()
                    .unwrap()
                    .send(game::CreatureAction::Die(entity));
            } else {
                let range = game::PositionRange::from(factors.magnet_shroom.range) + *pos;
                let entities = q_magnetic
                    .iter()
                    .filter_map(|(entity, parent, m_pos, m_hitbox)| {
                        let m_pos = if let Some(parent) = parent {
                            q_pos.get(parent.get()).cloned().unwrap_or_default() + *m_pos
                        } else {
                            *m_pos
                        };
                        if range.contains(&m_pos, m_hitbox) {
                            Some(MagnetEntity(pos.distance_squared(&m_pos), entity))
                        } else {
                            None
                        }
                    })
                    .collect::<BTreeSet<_>>();
                let mut len = 0;
                for MagnetEntity(_dist, magnetic) in entities.into_iter().take(diff) {
                    if let Ok((parent, image, magnetic_hitbox)) = q_armor.get(magnetic) {
                        commands.command_scope(|mut commands| {
                            if let Some(mut commands) = commands.get_entity(parent.get()) {
                                commands.remove_children(&[magnetic]);
                            }
                            commands
                                .spawn((
                                    game::Position::new_xy(
                                        rand::thread_rng().gen_range(-0.2..0.2),
                                        rand::thread_rng().gen_range(0.2..0.4),
                                    ),
                                    *magnetic_hitbox,
                                    SpriteBundle {
                                        texture: image.clone(),
                                        ..Default::default()
                                    },
                                    game::LayerDisp(-0.1),
                                ))
                                .set_parent(entity);
                            commands.entity(magnetic).despawn_recursive();
                        });
                        len += 1;
                    } else if q_digger.get(magnetic).is_ok() {
                        digger_go_up.lock().unwrap().insert(magnetic);
                        len += 1;
                    }
                }
                count.0 += len;
            }
        });
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: magnet_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .magnet_shroom
                .frames
                .first()
                .expect("Empty animation magnet_shroom")
                .clone(),
            cost: factors.magnet_shroom.cost,
            cooldown: factors.magnet_shroom.cooldown,
            hitbox: factors.magnet_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(MAGNET_SHROOM, creature);
    }
}
