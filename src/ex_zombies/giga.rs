use crate::prelude::*;

pub(super) struct ExZombiesGigaPlugin;

impl Plugin for ExZombiesGigaPlugin {
    fn build(&self, app: &mut App) {
        initialize(&giga_all_star_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (giga_tackle,).run_if(when_state!(gaming)));
        *giga_all_star_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_giga_all_star_zombie),
            ..Default::default()
        });
    }
}

#[derive(Component, Debug, Default)]
pub enum GigaAllStarZombieStatus {
    #[default]
    Running,
    Sliding,
    Walking,
}

#[derive(Resource, Debug)]
pub struct GigaAllStarInfo {
    pub acc: f32,
}

game_conf!(walker GigaAllStarZombieWalker);
game_conf!(breaks GigaHelmetBreaks);
game_conf!(systems giga_all_star_zombie_systems);

fn spawn_giga_all_star_zombie(
    In(pos): In<game::LogicPosition>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    breaks: Res<GigaHelmetBreaks>,
) {
    let creature = map.get(&GIGA_ALL_STAR_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(ex_factors.giga.velocity_running),
            sprite::Animation::new(ex_zombies.giga_all_star_running.clone()),
            compn::Dying::new(ex_zombies.giga_all_star_dying.clone()),
            creature.hitbox,
            GigaAllStarZombieStatus::default(),
            game::Health::from(ex_factors.giga.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.05, 0.0, 0.35, -0.25),
            ex_factors.giga.helmet_box,
            sprite::Animation::new(ex_zombies.giga_helmet.clone()),
            game::Armor::new(ex_factors.giga.helmet_health),
            game::Magnetic,
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.01),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

fn giga_tackle(
    commands: ParallelCommands,
    action: EventWriter<game::CreatureAction>,
    mut q_giga: Query<(
        Entity,
        &game::Overlay,
        &mut GigaAllStarZombieStatus,
        &mut game::Velocity,
        &mut game::VelocityBase,
        &mut sprite::Animation,
    )>,
    q_creature: Query<(), With<game::Creature>>,
    q_zombie: Query<(), With<game::Zombie>>,
    collision: Res<game::Collision>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    ex_zombies: Res<assets::SpriteExZombies>,
    walker: Res<GigaAllStarZombieWalker>,
    info: Res<GigaAllStarInfo>,
) {
    let action = Mutex::new(action);
    q_giga.par_iter_mut().for_each(
        |(entity, overlay, mut status, mut velocity, mut velocity_base, mut anim)| match *status {
            GigaAllStarZombieStatus::Running => {
                let is_zombie = q_zombie.get(entity).is_ok();
                let target = collision.get(&entity).and_then(|coll| {
                    coll.iter().find(|other| {
                        q_creature.get(**other).is_ok()
                            && (q_zombie.get(**other).is_ok() ^ is_zombie)
                    })
                });
                if let Some(target) = target {
                    action.lock().unwrap().send(game::CreatureAction::Damage(
                        *target,
                        ex_factors.giga.tackle_damage,
                    ));
                    *status = GigaAllStarZombieStatus::Sliding;
                    anim.replace(ex_zombies.giga_all_star_sliding.clone());
                }
            }
            GigaAllStarZombieStatus::Sliding => {
                if (velocity.x.abs() - ex_factors.giga.velocity_running.0 .1.abs()).abs()
                    >= (ex_factors.giga.velocity_running.0 .0 - ex_factors.giga.velocity.0 .0).abs()
                {
                    *status = GigaAllStarZombieStatus::Walking;
                    velocity_base.replace(ex_factors.giga.velocity.into());
                    anim.replace(ex_zombies.giga_all_star.clone());
                    commands.command_scope(|mut commands| {
                        if let Some(mut commands) = commands.get_entity(entity) {
                            commands.try_insert(compn::Walker(walker.0.clone()));
                        }
                    });
                } else {
                    velocity.x -= info.acc.copysign(velocity.x) * overlay.delta_secs();
                }
            }
            GigaAllStarZombieStatus::Walking => {}
        },
    );
}

fn init_config(
    mut commands: Commands,
    ex_zombies: Res<assets::SpriteExZombies>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(GigaAllStarZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.giga.interval),
        damage: ex_factors.giga.damage,
    })));
    commands.insert_resource(GigaHelmetBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            ex_zombies.giga_helmet.clone(),
            ex_zombies.giga_helmet_damaged.clone(),
            ex_zombies.giga_helmet_destroyed.clone(),
        ],
        init: ex_factors.giga.helmet_health,
    })));
    {
        let square = |x| x * x;
        let diff =
            square(ex_factors.giga.velocity_running.0 .1) - square(ex_factors.giga.velocity.0 .0);
        let acc = (diff / ex_factors.giga.slide_distance / 2.0).abs();
        commands.insert_resource(GigaAllStarInfo { acc });
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: GIGA_ALL_STAR_ZOMBIE,
            systems: giga_all_star_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies.giga_all_star_concept.clone(),
            cost: ex_factors.giga.cost,
            cooldown: ex_factors.giga.cooldown,
            hitbox: ex_factors.giga.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
}
