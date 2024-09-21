use crate::prelude::*;

pub(super) struct PlantsFumePlugin;

impl Plugin for PlantsFumePlugin {
    fn build(&self, app: &mut App) {
        initialize(&fume_shroom_systems);
        initialize(&fume_shroom_after);
        initialize(&fume_shroom_callback);
        app.add_systems(PostStartup, (init_config,));
        *fume_shroom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_fume_shroom),
            ..Default::default()
        });
        *fume_shroom_after.write().unwrap() = Some(app.register_system(move_fume));
        *fume_shroom_callback.write().unwrap() = Some(app.register_system(play_shoot_animation));
    }
}

game_conf!(projectile ProjectileFume);
game_conf!(shooter FumeShroomShooter);
game_conf!(systems fume_shroom_systems);
game_conf!(system fume_shroom_after, Entity);
game_conf!(system fume_shroom_callback, Entity);

fn spawn_fume_shroom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<FumeShroomShooter>,
) {
    let creature = map.get(&FUME_SHROOM).unwrap();
    commands.spawn((
        game::Plant,
        compn::Mushroom::default(),
        creature.clone(),
        pos,
        sprite::Animation::new(plants.fume_shroom.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.fume_shroom.health),
        SpriteBundle::default(),
    ));
}

fn move_fume(
    In(entity): In<Entity>,
    factors: Res<plants::PlantFactors>,
    mut q_pos: Query<&mut game::LogicPosition>,
) {
    if let Ok(mut pos) = q_pos.get_mut(entity) {
        pos.plus_assign(game::Position::new_xy(
            factors.fume_shroom.fume_box.width / 2.0,
            0.0,
        ));
    }
}

fn play_shoot_animation(
    In(entity): In<Entity>,
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(compn::AnimationThenDo {
            anim: plants.fume_shroom_shoot.clone(),
            ..Default::default()
        });
    }
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    audio_plants: Res<assets::AudioPlants>,
    mut map: ResMut<game::CreatureMap>,
) {
    let fume = Arc::new(game::ProjectileShared {
        anim: plants.fume.clone(),
        hitbox: factors.fume_shroom.fume_box,
    });
    commands.insert_resource(ProjectileFume(fume.clone()));
    {
        commands.insert_resource(FumeShroomShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.fume_shroom.interval),
            velocity: Default::default(),
            proj: game::Projectile {
                damage: factors.fume_shroom.damage,
                area: true,
                // NOTE: Do we need to allow customizing the time fume stays on the ground?
                time: Duration::from_secs_f32(0.4),
                range: game::PositionRangeX(factors.fume_shroom.fume_box.width).into(),
                ..Default::default()
            },
            times: factors.fume_shroom.times,
            require_zombie: compn::RequireZombie::InRange,
            after: fume_shroom_after.read().unwrap().unwrap(),
            callback: fume_shroom_callback.read().unwrap().unwrap(),
            shared: fume.clone(),
            audio: audio_plants.fume_shroom.clone(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: FUME_SHROOM,
            systems: fume_shroom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .fume_shroom
                .frames
                .first()
                .expect("Empty animation fume_shroom")
                .clone(),
            cost: factors.fume_shroom.cost,
            cooldown: factors.fume_shroom.cooldown,
            hitbox: factors.fume_shroom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
