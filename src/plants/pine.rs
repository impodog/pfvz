use crate::prelude::*;

pub(super) struct PlantsPinePlugin;

impl Plugin for PlantsPinePlugin {
    fn build(&self, app: &mut App) {
        initialize(&sap_fling_systems);
        initialize(&pine_after);
        initialize(&sap_fling_callback);
        app.add_systems(PostStartup, (init_config,));
        *sap_fling_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_sap_fling),
            ..Default::default()
        });
        *pine_after.write().unwrap() = Some(app.register_system(add_gravity_and_snow));
        *sap_fling_callback.write().unwrap() = Some(app.register_system(sap_fling_play_animation));
    }
}

game_conf!(projectile ProjectilePine);
game_conf!(shooter SapFlingShooter);
game_conf!(systems sap_fling_systems);
game_conf!(pub system pine_after, Entity);
game_conf!(system sap_fling_callback, Entity);

fn add_gravity_and_snow(
    In(entity): In<Entity>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands
            .try_insert(game::Gravity)
            .try_insert(compn::SnowyProjectile {
                snow: factors.sap_fling.snow.into(),
                range: Some(factors.sap_fling.range.into()),
            });
    }
}

fn spawn_sap_fling(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<SapFlingShooter>,
) {
    let creature = map.get(&SAP_FLING).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.sap_fling.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.sap_fling.health),
        SpriteBundle::default(),
    ));
}

fn sap_fling_play_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: plants.sap_fling_lob.clone(),
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
    let pine = Arc::new(game::ProjectileShared {
        anim: plants.pine.clone(),
        hitbox: factors.sap_fling.pine_box,
    });
    commands.insert_resource(ProjectilePine(pine.clone()));
    {
        commands.insert_resource(SapFlingShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.sap_fling.interval),
            velocity: factors.sap_fling.velocity.into(),
            proj: game::Projectile {
                damage: factors.sap_fling.damage,
                area: true,
                range: game::PositionRange::default().with_inf_z(),
                ..Default::default()
            },
            times: factors.sap_fling.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: pine.clone(),
            after: pine_after.read().unwrap().unwrap(),
            callback: sap_fling_callback.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: SAP_FLING,
            systems: sap_fling_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .sap_fling
                .frames
                .first()
                .expect("Empty animation sap_fling")
                .clone(),
            cost: factors.sap_fling.cost,
            cooldown: factors.sap_fling.cooldown,
            hitbox: factors.sap_fling.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
