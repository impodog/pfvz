use crate::prelude::*;

pub(super) struct ZombiesTrashcanPlugin;

impl Plugin for ZombiesTrashcanPlugin {
    fn build(&self, app: &mut App) {
        initialize(&trashcan_zombie_systems);
        initialize(&trashcan_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (trashcan_stop,));
        *trashcan_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_trashcan_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *trashcan_systems.write().unwrap() = Some(game::CreatureSystems {
            // NOTE: Maybe we should not spawn trashcan zombie in trashcan systems
            spawn: app.register_system(spawn_trashcan_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems trashcan_zombie_systems);
game_conf!(walker TrashcanZombieWalker);
game_conf!(systems trashcan_systems);
game_conf!(breaks TrashcanBreaks);
game_conf!(walker TrashcanWalker);

#[derive(Component, Debug, Clone)]
struct TrashcanBind(Entity);

fn spawn_trashcan_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    self_walker: Res<TrashcanZombieWalker>,
    trashcan_walker: Res<TrashcanWalker>,
    breaks: Res<TrashcanBreaks>,
) {
    let creature = map.get(&TRASHCAN_ZOMBIE).unwrap();
    let velocity = game::Velocity::from(factors.trashcan.velocity);
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            velocity,
            sprite::Animation::new(zombies.trashcan_zombie.clone()),
            compn::Dying::new(zombies.trashcan_zombie_dying.clone()),
            creature.hitbox,
            compn::Walker(self_walker.0.clone()),
            game::Health::from(factors.trashcan.self_health),
            SpriteBundle::default(),
        ))
        .id();
    let trashcan = map.get(&TRASHCAN).unwrap();
    let disp = game::Position {
        x: -(factors.trashcan.self_box.width + factors.trashcan.trashcan_box.width) / 2.0,
        ..Default::default()
    };
    commands.spawn((
        game::Zombie,
        game::NotInvasive,
        trashcan.clone(),
        pos.plus(disp),
        velocity,
        sprite::Animation::new(zombies.trashcan.clone()),
        trashcan.hitbox,
        compn::Walker(trashcan_walker.0.clone()),
        compn::Breaks(breaks.0.clone()),
        TrashcanBind(entity),
        game::Health::from(factors.trashcan.trashcan_health),
        SpriteBundle::default(),
    ));
}

fn trashcan_stop(
    mut commands: Commands,
    mut q_trashcan: Query<(
        Entity,
        &TrashcanBind,
        &mut compn::WalkerImpl,
        &mut game::Velocity,
    )>,
) {
    q_trashcan
        .iter_mut()
        .for_each(|(entity, bind, mut walker_impl, mut velocity)| {
            if commands.get_entity(bind.0).is_none() {
                commands.entity(entity).remove::<TrashcanBind>();
                walker_impl.target = None;
                velocity.x = 0.0;
            }
        });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(TrashcanZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.trashcan.interval),
        damage: factors.trashcan.damage,
    })));
    commands.insert_resource(TrashcanWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.trashcan.interval),
        damage: factors.trashcan.trashcan_damage,
    })));
    commands.insert_resource(TrashcanBreaks(Arc::new(compn::BreaksShared {
        v: vec![zombies.trashcan.clone(), zombies.trashcan_broken.clone()],
        init: factors.trashcan.trashcan_health.0,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: trashcan_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .trashcan_zombie
                .frames
                .first()
                .expect("empty animation trashcan_zombie")
                .clone(),
            cost: factors.trashcan.cost,
            cooldown: factors.trashcan.cooldown,
            hitbox: factors.trashcan.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(TRASHCAN_ZOMBIE, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: trashcan_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .trashcan
                .frames
                .first()
                .expect("empty animation trashcan")
                .clone(),
            cost: factors.trashcan.cost,
            cooldown: factors.trashcan.cooldown,
            hitbox: factors.trashcan.trashcan_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(TRASHCAN, creature);
    }
}
