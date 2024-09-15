use crate::prelude::*;

pub(super) struct PlantsStarfruitPlugin;

impl Plugin for PlantsStarfruitPlugin {
    fn build(&self, app: &mut App) {
        initialize(&starfruit_systems);
        app.add_systems(PostStartup, (init_config,));
        *starfruit_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_puff_shroom),
            ..Default::default()
        });
    }
}

game_conf!(systems starfruit_systems);
game_conf!(shooter StarfruitShooter);
game_conf!(projectile ProjectileStar);

fn spawn_puff_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<StarfruitShooter>,
) {
    let creature = map.get(&STARFRUIT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.starfruit.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.starfruit.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let star = Arc::new(game::ProjectileShared {
        anim: plants.star.clone(),
        hitbox: factors.starfruit.star_box,
    });
    commands.insert_resource(ProjectileStar(star.clone()));
    {
        let front_angle = 1.0f32.atan2(2.0);
        commands.insert_resource(StarfruitShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.starfruit.interval),
            velocity: factors.starfruit.velocity.into(),
            proj: game::Projectile {
                damage: factors.starfruit.damage,
                ..Default::default()
            },
            start: vec![
                (game::Position::default(), front_angle),
                (game::Position::default(), -front_angle),
                (game::Position::default(), std::f32::consts::FRAC_PI_2),
                (game::Position::default(), -std::f32::consts::FRAC_PI_2),
                (game::Position::default(), std::f32::consts::PI),
            ],
            times: factors.starfruit.times,
            require_zombie: compn::RequireZombie::RayCast,
            shared: star.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: starfruit_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .starfruit
                .frames
                .first()
                .expect("Empty animation starfruit")
                .clone(),
            cost: factors.starfruit.cost,
            cooldown: factors.starfruit.cooldown,
            hitbox: factors.starfruit.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(STARFRUIT, creature);
    }
}
