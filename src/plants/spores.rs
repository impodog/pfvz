use crate::prelude::*;

pub(super) struct PlantsSporesPlugin;

impl Plugin for PlantsSporesPlugin {
    fn build(&self, app: &mut App) {
        initialize(&puff_shroom_systems);
        initialize(&scaredy_shroom_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (modify_scaredy,));
        *puff_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_puff_shroom),
            ..Default::default()
        });
        *scaredy_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_scaredy_shroom),
            ..Default::default()
        });
    }
}

game_conf!(projectile ProjectileSpore);
game_conf!(shooter PuffShroomShooter);
game_conf!(systems puff_shroom_systems);
game_conf!(shooter ScaredyShroomShooter);
game_conf!(systems scaredy_shroom_systems);

fn spawn_puff_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<PuffShroomShooter>,
) {
    let creature = map.get(&PUFF_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.puff_shroom.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.puff_shroom.health),
        SpriteBundle::default(),
    ));
}

#[derive(Component, Debug, Clone, Default, Deref, DerefMut)]
pub struct ScaredyStatus(pub bool);

fn spawn_scaredy_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<ScaredyShroomShooter>,
) {
    let creature = map.get(&SCAREDY_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.scaredy_shroom.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        ScaredyStatus::default(),
        game::Velocity::default(),
        game::Health::from(factors.scaredy_shroom.health),
        SpriteBundle::default(),
    ));
}

fn modify_scaredy(
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    mut q_scaredy: Query<(
        Entity,
        &game::Position,
        &mut ScaredyStatus,
        &mut game::Velocity,
    )>,
    q_zombie: Query<(&game::Position, &game::HitBox), (With<game::Zombie>, Without<ScaredyStatus>)>,
    shooter: Res<ScaredyShroomShooter>,
) {
    q_scaredy
        .iter_mut()
        .for_each(|(entity, pos, mut scaredy, mut velocity)| {
            let range = factors.scaredy_shroom.scare_range.clone() + *pos;
            let mut is_scaredy = false;
            for (zombie_pos, zombie_hitbox) in q_zombie.iter() {
                if range.contains(zombie_pos, zombie_hitbox) {
                    is_scaredy = true;
                    break;
                }
            }
            velocity.r = if is_scaredy { -6.2 } else { 0.0 };
            if scaredy.0 != is_scaredy {
                if is_scaredy {
                    commands.entity(entity).remove::<compn::Shooter>();
                } else {
                    commands
                        .entity(entity)
                        .insert(compn::Shooter(shooter.0.clone()));
                }
                scaredy.0 = is_scaredy;
            }
        });
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    audio_plants: Res<assets::AudioPlants>,
    mut map: ResMut<game::CreatureMap>,
) {
    let spore = Arc::new(game::ProjectileShared {
        anim: plants.spore.clone(),
        hitbox: factors.puff_shroom.spore_box,
    });
    commands.insert_resource(ProjectileSpore(spore.clone()));
    {
        commands.insert_resource(PuffShroomShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.puff_shroom.interval),
            velocity: factors.puff_shroom.velocity.into(),
            proj: game::Projectile {
                damage: factors.puff_shroom.damage,
                range: factors.puff_shroom.range.into(),
                ..Default::default()
            },
            times: factors.puff_shroom.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: spore.clone(),
            audio: audio_plants.spore.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: PUFF_SHROOM,
            systems: puff_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .puff_shroom
                .frames
                .first()
                .expect("Empty animation puff_shroom")
                .clone(),
            cost: factors.puff_shroom.cost,
            cooldown: factors.puff_shroom.cooldown,
            hitbox: factors.puff_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
    {
        commands.insert_resource(ScaredyShroomShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.scaredy_shroom.interval),
            velocity: factors.scaredy_shroom.velocity.into(),
            proj: game::Projectile {
                damage: factors.scaredy_shroom.damage,
                ..Default::default()
            },
            times: factors.scaredy_shroom.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: spore.clone(),
            audio: audio_plants.spore.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: SCAREDY_SHROOM,
            systems: scaredy_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .scaredy_shroom
                .frames
                .first()
                .expect("Empty animation scaredy_shroom")
                .clone(),
            cost: factors.scaredy_shroom.cost,
            cooldown: factors.scaredy_shroom.cooldown,
            hitbox: factors.scaredy_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
