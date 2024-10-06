use crate::prelude::*;

pub(super) struct ZombiesSnorkelPlugin;

impl Plugin for ZombiesSnorkelPlugin {
    fn build(&self, app: &mut App) {
        initialize(&snorkel_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (snorkel_enter,).run_if(when_state!(gaming)));
        *snorkel_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_snorkel_zombie),
            ..Default::default()
        });
    }
}

game_conf!(systems snorkel_zombie_systems);
game_conf!(walker SnorkelZombieWalker);

fn spawn_snorkel_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    walker: Res<SnorkelZombieWalker>,
) {
    let creature = map.get(&SNORKEL_ZOMBIE).unwrap();
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::from(factors.snorkel.velocity),
        sprite::Animation::new(zombies.snorkel_zombie.clone()),
        compn::Dying::new(zombies.snorkel_zombie_dying.clone()),
        creature.hitbox,
        compn::Walker(walker.0.clone()),
        SnorkelStatus::default(),
        game::Health::from(factors.snorkel.self_health),
        SpriteBundle::default(),
    ));
}

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
struct SnorkelStatus(bool);

fn snorkel_enter(
    mut q_snorkel: Query<(
        Entity,
        &game::LogicPosition,
        &mut SnorkelStatus,
        &mut game::HitBox,
        &mut game::SizeCrop,
    )>,
    q_walker_impl: Query<&compn::WalkerImpl>,
    level: Res<level::Level>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_snorkel
        .par_iter_mut()
        .for_each(|(entity, pos, mut status, mut hitbox, mut size)| {
            let (x, y) = level
                .config
                .layout
                .position_3d_to_coordinates(pos.base_raw());
            let diving = if level.config.layout.get_tile(x, y) == level::TileFeature::Water {
                !q_walker_impl
                    .get(entity)
                    .is_ok_and(|walker_impl| walker_impl.target.is_some())
            } else {
                false
            };
            if status.0 != diving {
                status.0 = diving;
                let stretch_factor =
                    factors.snorkel.underwater_box.height / factors.snorkel.self_box.height;
                let crop_factor = stretch_factor / WATER_PERCENTAGE;
                if diving {
                    *hitbox = factors.snorkel.underwater_box;
                    size.y_crop.multiply(crop_factor);
                    size.y_stretch.multiply(1.0 / stretch_factor);
                } else {
                    *hitbox = factors.snorkel.self_box;
                    size.y_crop.divide(crop_factor);
                    size.y_stretch.divide(1.0 / stretch_factor);
                }
            }
        });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(SnorkelZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.snorkel.interval),
        damage: factors.snorkel.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: SNORKEL_ZOMBIE,
            systems: snorkel_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .snorkel_zombie
                .frames
                .first()
                .expect("empty animation snorkel_zombie")
                .clone(),
            cost: factors.newspaper.cost,
            cooldown: factors.snorkel.cooldown,
            hitbox: factors.snorkel.self_box,
            flags: level::CreatureFlags::AQUATIC_ZOMBIE,
        }));
        map.insert(creature);
    }
}
