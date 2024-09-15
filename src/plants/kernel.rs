use crate::prelude::*;

pub(super) struct PlantsKernelPlugin;

impl Plugin for PlantsKernelPlugin {
    fn build(&self, app: &mut App) {
        initialize(&kernel_pult_systems);
        initialize(&kernel_callback);
        app.add_systems(PostStartup, (init_config,));
        *kernel_pult_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_kernel_pult),
            ..Default::default()
        });
        *kernel_after.write().unwrap() = Some(app.register_system(add_gravity_and_butter));
        *kernel_callback.write().unwrap() = Some(app.register_system(kernel_play_animation));
    }
}

game_conf!(projectile ProjectileKernel);
game_conf!(projectile ProjectileButter);
game_conf!(shooter KernelShooter);
game_conf!(shooter ButterShooter);
game_conf!(systems kernel_pult_systems);
game_conf!(system kernel_after, Entity);
game_conf!(system kernel_callback, Entity);

#[derive(Component, Debug, Deref, DerefMut, Default)]
pub struct KernelPultCount(pub usize);

fn spawn_kernel_pult(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<KernelShooter>,
) {
    let creature = map.get(&KERNEL_PULT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.kernel_pult.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        KernelPultCount::default(),
        game::Health::from(factors.kernel_pult.health),
        SpriteBundle::default(),
    ));
}

fn add_gravity_and_butter(
    In(entity): In<Entity>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(game::Gravity);
        commands.try_insert(compn::SnowyProjectile {
            snow: factors.kernel_pult.snow.into(),
            ..Default::default()
        });
    }
}

fn kernel_play_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    mut q_count: Query<(&mut KernelPultCount, &mut compn::Shooter)>,
    factors: Res<plants::PlantFactors>,
    kernel: Res<KernelShooter>,
    butter: Res<ButterShooter>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: plants.kernel_pult_lob.clone(),
            ..Default::default()
        });
    }
    if let Ok((mut count, mut shooter)) = q_count.get_mut(entity) {
        count.0 += 1;
        if count.0 >= factors.kernel_pult.butter_every {
            shooter.replace(butter.0.clone());
            count.0 = 0;
        } else if count.0 == 1 {
            shooter.replace(kernel.0.clone());
        }
    }
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let kernel = Arc::new(game::ProjectileShared {
        anim: plants.kernel.clone(),
        hitbox: factors.kernel_pult.kernel_box,
    });
    let butter = Arc::new(game::ProjectileShared {
        anim: plants.butter.clone(),
        hitbox: factors.kernel_pult.butter_box,
    });
    commands.insert_resource(ProjectileKernel(kernel.clone()));
    commands.insert_resource(ProjectileButter(butter.clone()));
    {
        commands.insert_resource(KernelShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.kernel_pult.interval),
            velocity: factors.kernel_pult.velocity.into(),
            proj: game::Projectile {
                damage: factors.kernel_pult.kernel_damage,
                range: game::PositionRange::default().with_inf_z(),
                ..Default::default()
            },
            times: factors.kernel_pult.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: kernel.clone(),
            after: plants::add_gravity_system.read().unwrap().unwrap(),
            callback: kernel_callback.read().unwrap().unwrap(),
            ..Default::default()
        })));
        commands.insert_resource(ButterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.kernel_pult.interval),
            velocity: factors.kernel_pult.velocity.into(),
            proj: game::Projectile {
                damage: factors.kernel_pult.butter_damage,
                range: game::PositionRange::default().with_inf_z(),
                ..Default::default()
            },
            times: factors.kernel_pult.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: butter.clone(),
            after: kernel_after.read().unwrap().unwrap(),
            callback: kernel_callback.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: kernel_pult_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .kernel_pult
                .frames
                .first()
                .expect("Empty animation kernel_pult")
                .clone(),
            cost: factors.kernel_pult.cost,
            cooldown: factors.kernel_pult.cooldown,
            hitbox: factors.kernel_pult.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(KERNEL_PULT, creature);
    }
}
