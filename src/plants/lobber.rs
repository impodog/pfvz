use crate::prelude::*;

pub(super) struct PlantsLobberPlugin;

impl Plugin for PlantsLobberPlugin {
    fn build(&self, app: &mut App) {
        initialize(&cabbage_pult_systems);
        app.add_systems(PostStartup, (init_config,));
        *cabbage_pult_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_cabbage_pult),
            ..Default::default()
        });
        *add_gravity_system.write().unwrap() = Some(app.register_system(add_gravity));
        *cabbage_callback.write().unwrap() = Some(app.register_system(cabbage_play_animation));
    }
}

game_conf!(projectile ProjectileCabbage);
game_conf!(shooter CabbagePultShooter);
game_conf!(systems cabbage_pult_systems);
game_conf!(pub system add_gravity_system, Entity);
game_conf!(system cabbage_callback, Entity);

fn add_gravity(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(game::Gravity);
    }
}

fn spawn_cabbage_pult(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<CabbagePultShooter>,
) {
    let creature = map.get(&CABBAGE_PULT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.cabbage_pult.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.cabbage_pult.health),
        SpriteBundle::default(),
    ));
}

fn cabbage_play_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: plants.cabbage_pult_lob.clone(),
            ..Default::default()
        });
    }
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let cabbage = Arc::new(game::ProjectileShared {
        anim: plants.cabbage.clone(),
        hitbox: factors.cabbage_pult.cabbage_box,
    });
    commands.insert_resource(ProjectileCabbage(cabbage.clone()));
    {
        commands.insert_resource(CabbagePultShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.cabbage_pult.interval),
            velocity: factors.cabbage_pult.velocity.into(),
            proj: game::Projectile {
                damage: factors.cabbage_pult.damage,
                range: game::PositionRange::default().with_inf_z(),
                ..Default::default()
            },
            times: factors.cabbage_pult.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: cabbage.clone(),
            after: add_gravity_system.read().unwrap().unwrap(),
            callback: cabbage_callback.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: CABBAGE_PULT,
            systems: cabbage_pult_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .cabbage_pult
                .frames
                .first()
                .expect("Empty animation cabbage_pult")
                .clone(),
            cost: factors.cabbage_pult.cost,
            cooldown: factors.cabbage_pult.cooldown,
            hitbox: factors.cabbage_pult.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
