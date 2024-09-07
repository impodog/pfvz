use crate::prelude::*;

pub(super) struct PlantsMelonPlugin;

impl Plugin for PlantsMelonPlugin {
    fn build(&self, app: &mut App) {
        initialize(&melon_systems);
        app.add_systems(PostStartup, (init_config,));
        *melon_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_melon_pult),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *melon_after.write().unwrap() = Some(app.register_system(add_gravity_and_fire));
        *melon_callback.write().unwrap() = Some(app.register_system(melon_play_animation));
        *melon_consume.write().unwrap() = Some(app.register_system(play_melon_sound));
    }
}

game_conf!(projectile ProjectileMelon);
game_conf!(shooter MelonPultShooter);
game_conf!(systems melon_systems);
game_conf!(system melon_after, Entity);
game_conf!(system melon_callback, Entity);
game_conf!(system melon_consume, Entity);

fn spawn_melon_pult(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<MelonPultShooter>,
) {
    let creature = map.get(&MELON_PULT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.melon_pult.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.melon_pult.health),
        SpriteBundle::default(),
    ));
}

fn play_melon_sound(
    In(_entity): In<Entity>,
    audio: Res<Audio>,
    audio_plants: Res<assets::AudioPlants>,
) {
    audio.play(audio_plants.melon.random());
}

fn add_gravity_and_fire(
    In(entity): In<Entity>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    mut e_fire: EventWriter<compn::ModifyFire>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands
            .try_insert(game::Gravity)
            .try_insert(compn::ProjectileConsume(
                melon_consume.read().unwrap().unwrap(),
            ));
        e_fire.send(compn::ModifyFire::Add(
            entity,
            factors.melon_pult.fire.into(),
        ));
    }
}

fn melon_play_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: plants.melon_pult_lob.clone(),
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
    let melon = Arc::new(game::ProjectileShared {
        anim: plants.melon.clone(),
        hitbox: factors.melon_pult.melon_box,
    });
    commands.insert_resource(ProjectileMelon(melon.clone()));
    {
        commands.insert_resource(MelonPultShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.melon_pult.interval),
            velocity: factors.melon_pult.velocity.into(),
            proj: game::Projectile {
                damage: factors.melon_pult.damage,
                range: game::PositionRange::default().with_inf_z(),
                ..Default::default()
            },
            times: factors.melon_pult.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: melon.clone(),
            after: melon_after.read().unwrap().unwrap(),
            callback: melon_callback.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: melon_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .melon_pult
                .frames
                .first()
                .expect("Empty animation melon_pult")
                .clone(),
            cost: factors.melon_pult.cost,
            cooldown: factors.melon_pult.cooldown,
            hitbox: factors.melon_pult.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(MELON_PULT, creature);
    }
}
