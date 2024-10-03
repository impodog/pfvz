use crate::prelude::*;

pub(super) struct ExPlantsThistlePlugin;

impl Plugin for ExPlantsThistlePlugin {
    fn build(&self, app: &mut App) {
        initialize(&homing_thistle_systems);
        initialize(&homing_thistle_after);
        app.add_systems(PostStartup, (init_config,));
        *homing_thistle_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_homing_thistle),
            ..Default::default()
        });
        *homing_thistle_after.write().unwrap() =
            Some(app.register_system(homing_thistle_add_aiming));
    }
}

game_conf!(systems homing_thistle_systems);
game_conf!(system homing_thistle_after, Entity);
game_conf!(shooter HomingThistleShooter);
game_conf!(projectile ProjectilePrick);

fn spawn_homing_thistle(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    ex_factors: Res<ex_plants::ExPlantFactors>,
    ex_plants: Res<assets::SpriteExPlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<HomingThistleShooter>,
) {
    let creature = map.get(&HOMING_THISTLE).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(ex_plants.homing_thistle.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(ex_factors.homing_thistle.health),
        SpriteBundle::default(),
    ));
}

fn homing_thistle_add_aiming(
    In(entity): In<Entity>,
    mut commands: Commands,
    ex_factors: Res<ex_plants::ExPlantFactors>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::Aiming {
            range: ex_factors.homing_thistle.range.into(),
            ..Default::default()
        });
    }
}

fn init_config(
    mut commands: Commands,
    ex_plants: Res<assets::SpriteExPlants>,
    ex_factors: Res<ex_plants::ExPlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let prick = Arc::new(game::ProjectileShared {
        anim: ex_plants.prick.clone(),
        hitbox: ex_factors.homing_thistle.prick_box,
    });
    commands.insert_resource(ProjectilePrick(prick.clone()));
    {
        let range = ex_factors.homing_thistle.range.into();
        commands.insert_resource(HomingThistleShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(ex_factors.homing_thistle.interval),
            velocity: ex_factors.homing_thistle.velocity.into(),
            proj: game::Projectile {
                damage: ex_factors.homing_thistle.damage,
                range,
                ..Default::default()
            },
            after: homing_thistle_after.read().unwrap().unwrap(),
            times: ex_factors.homing_thistle.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: prick.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: HOMING_THISTLE,
            systems: homing_thistle_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_plants
                .homing_thistle
                .frames
                .first()
                .expect("Empty animation homing_thistle")
                .clone(),
            cost: ex_factors.homing_thistle.cost,
            cooldown: ex_factors.homing_thistle.cooldown,
            hitbox: ex_factors.homing_thistle.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_AQUATIC_PLANT,
        }));
        map.insert(creature);
    }
}
