use crate::prelude::*;

pub(super) struct PlantsPeaPlugin;

impl Plugin for PlantsPeaPlugin {
    fn build(&self, app: &mut App) {
        initialize(&peashooter_systems);
        app.add_systems(PostStartup, (init_config,));
        *peashooter_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_peashooter),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });

        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_spawn_system!(PEASHOOTER, -4.0, 0.0));
    }
}

game_conf!(projectile ProjectilePea);
game_conf!(shooter PeashooterShooter);
game_conf!(systems peashooter_systems);

fn spawn_peashooter(
    In(pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    map: Res<game::CreatureMap>,
    shooter: Res<PeashooterShooter>,
) {
    let creature = map.get(&PEASHOOTER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(creature.anim.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.peashooter.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let pea = Arc::new(game::ProjectileShared {
        anim: plants.pea.clone(),
        hitbox: factors.peashooter.pea_box,
    });
    commands.insert_resource(ProjectilePea(pea.clone()));
    {
        commands.insert_resource(PeashooterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_millis(factors.peashooter.interval),
            velocity: factors.peashooter.velocity.into(),
            proj: game::Projectile {
                damage: factors.peashooter.damage,
                instant: true,
            },
            require_zombie: true,
            shared: pea.clone(),
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: peashooter_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            anim: plants.peashooter.clone(),
            cost: factors.peashooter.cost,
            hitbox: factors.peashooter.self_box,
        }));
        map.insert(PEASHOOTER, creature);
    }
}
