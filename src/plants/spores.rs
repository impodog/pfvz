use crate::prelude::*;

pub(super) struct PlantsSporesPlugin;

impl Plugin for PlantsSporesPlugin {
    fn build(&self, app: &mut App) {
        initialize(&puff_shroom_systems);
        app.add_systems(PostStartup, (init_config,));
        *puff_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_puff_shroom),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *puff_shroom_after.write().unwrap() = Some(app.register_system(compn::default::do_nothing));
    }
}

game_conf!(projectile ProjectileSpore);
game_conf!(shooter PuffShroomShooter);
game_conf!(systems puff_shroom_systems);
game_conf!(system puff_shroom_after, Entity);

fn spawn_puff_shroom(
    In(pos): In<game::Position>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<PuffShroomShooter>,
) {
    let creature = map.get(&PUFF_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.puff_shroom.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.puff_shroom.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let spore = Arc::new(game::ProjectileShared {
        anim: plants.spore.clone(),
        hitbox: factors.puff_shroom.spore_box,
    });
    {
        commands.insert_resource(ProjectileSpore(spore.clone()));
        commands.insert_resource(PuffShroomShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.puff_shroom.interval),
            velocity: factors.puff_shroom.velocity.into(),
            proj: game::Projectile {
                damage: factors.puff_shroom.damage,
                instant: true,
                range: factors.puff_shroom.range.into(),
            },
            times: factors.puff_shroom.times,
            require_zombie: true,
            after: puff_shroom_after.read().unwrap().unwrap(),
            shared: spore.clone(),
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
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
            flags: level::CreatureFlags::TERRESTRIAL_CREATURE,
        }));
        map.insert(PUFF_SHROOM, creature);
    }
}
