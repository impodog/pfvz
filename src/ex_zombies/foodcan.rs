use crate::prelude::*;

pub(super) struct ExZombiesFoodcanPlugin;

impl Plugin for ExZombiesFoodcanPlugin {
    fn build(&self, app: &mut App) {
        initialize(&foodcan_zombie_systems);
        initialize(&foodcan_systems);
        app.add_systems(PostStartup, (init_config,));
        *foodcan_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_foodcan_zombie),
            ..Default::default()
        });
        *foodcan_systems.write().unwrap() = Some(game::CreatureSystems {
            // NOTE: Maybe we should not spawn trashcan zombie in trashcan systems
            spawn: app.register_system(spawn_foodcan_zombie),
            damage: app.register_system(foodcan_damage),
            ..Default::default()
        });
    }
}

game_conf!(systems foodcan_zombie_systems);
game_conf!(walker FoodcanZombieWalker);
game_conf!(systems foodcan_systems);
game_conf!(breaks FoodcanBreaks);
game_conf!(walker FoodcanWalker);

fn spawn_foodcan_zombie(
    In(pos): In<game::LogicPosition>,
    ex_zombies: Res<assets::SpriteExZombies>,
    mut commands: Commands,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    map: Res<game::CreatureMap>,
    self_walker: Res<FoodcanZombieWalker>,
    foodcan_walker: Res<FoodcanWalker>,
    breaks: Res<FoodcanBreaks>,
) {
    let creature = map.get(&FOODCAN_ZOMBIE).unwrap();
    let velocity = game::Velocity::from(ex_factors.foodcan.velocity);
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            velocity,
            sprite::Animation::new(ex_zombies.foodcan_zombie.clone()),
            compn::Dying::new(ex_zombies.foodcan_zombie_dying.clone()),
            creature.hitbox,
            compn::Walker(self_walker.0.clone()),
            game::Health::from(ex_factors.foodcan.self_health),
            SpriteBundle::default(),
        ))
        .id();
    let foodcan = map.get(&FOODCAN).unwrap();
    let disp = game::Position {
        x: -(ex_factors.foodcan.self_box.width + ex_factors.foodcan.foodcan_box.width) / 2.0,
        ..Default::default()
    };
    commands.spawn((
        game::Zombie,
        game::NotInvasive,
        foodcan.clone(),
        pos.plus(disp),
        velocity,
        sprite::Animation::new(ex_zombies.foodcan.clone()),
        foodcan.hitbox,
        compn::Walker(foodcan_walker.0.clone()),
        compn::Breaks(breaks.0.clone()),
        zombies::TrashcanBind(entity),
        game::Health::from(ex_factors.foodcan.foodcan_health),
        SpriteBundle::default(),
    ));
}

fn foodcan_damage(
    In((entity, damage)): In<(Entity, u32)>,
    mut commands: Commands,
    chunks: Res<assets::SpriteChunks>,
    q_health: Query<&game::Health>,
    q_pos: Query<(&game::Position, &game::HitBox)>,
) {
    let ok = if let Ok(health) = q_health.get(entity) {
        damage >= health.value()
    } else {
        true
    };
    if ok {
        commands.run_system_with_input(
            compn::default::system_damage.read().unwrap().unwrap(),
            (entity, damage),
        );
    } else if let Ok((pos, hitbox)) = q_pos.get(entity) {
        commands.spawn((
            *pos,
            *hitbox,
            level::Banner::new(Duration::from_millis(100)),
            SpriteBundle {
                texture: chunks.cross.clone(),
                ..Default::default()
            },
        ));
    }
}

fn init_config(
    mut commands: Commands,
    ex_zombies: Res<assets::SpriteExZombies>,
    ex_factors: Res<ex_zombies::ExZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(FoodcanZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.foodcan.interval),
        damage: ex_factors.foodcan.damage,
    })));
    commands.insert_resource(FoodcanWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(ex_factors.foodcan.interval),
        damage: ex_factors.foodcan.foodcan_damage,
    })));
    commands.insert_resource(FoodcanBreaks(Arc::new(compn::BreaksShared {
        v: vec![
            ex_zombies.foodcan.clone(),
            ex_zombies.foodcan_damaged.clone(),
        ],
        init: ex_factors.foodcan.foodcan_health.0,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: FOODCAN_ZOMBIE,
            systems: foodcan_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies
                .foodcan_zombie
                .frames
                .first()
                .expect("empty animation foodcan_zombie")
                .clone(),
            cost: ex_factors.foodcan.cost,
            cooldown: ex_factors.foodcan.cooldown,
            hitbox: ex_factors.foodcan.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: FOODCAN,
            systems: foodcan_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: ex_zombies
                .foodcan
                .frames
                .first()
                .expect("empty animation foodcan")
                .clone(),
            cost: ex_factors.foodcan.cost,
            cooldown: ex_factors.foodcan.cooldown,
            hitbox: ex_factors.foodcan.foodcan_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
}
