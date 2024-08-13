use crate::prelude::*;

pub(super) struct ZombiesHiddenPlugin;

impl Plugin for ZombiesHiddenPlugin {
    fn build(&self, app: &mut App) {
        initialize(&hidden_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (hidden_zombie_move,));
        *hidden_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_hidden_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems hidden_zombie_systems);
game_conf!(walker HiddenZombieWalker);

#[derive(Component)]
pub struct HiddenZombieMarker;

fn spawn_hidden_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    self_walker: Res<HiddenZombieWalker>,
) {
    let creature = map.get(&HIDDEN_ZOMBIE).unwrap();
    let velocity = game::Velocity::from(factors.hidden.velocity);
    commands.spawn((
        game::Zombie,
        HiddenZombieMarker,
        creature.clone(),
        pos,
        velocity,
        sprite::Animation::new(zombies.hidden_zombie.clone()),
        creature.hitbox,
        compn::Walker(self_walker.0.clone()),
        game::Health::from(factors.hidden.self_health),
        SpriteBundle::default(),
    ));
}

fn hidden_zombie_move(
    mut q_pos: Query<&mut game::LogicPosition, With<HiddenZombieMarker>>,
    level: Res<level::Level>,
) {
    q_pos.iter_mut().for_each(|mut pos| {
        if rand::thread_rng().gen_bool(0.01) {
            let prev_x = pos.base_raw().x;
            let (x, mut y) = level.config.layout.position_to_coordinates(pos.base_raw());
            if y == 0 {
                y += 1;
            } else if y >= level.config.layout.size().1 - 1 {
                y -= 1;
            } else if rand::thread_rng().gen_bool(0.5) {
                y += 1;
            } else {
                y -= 1;
            }
            let mut base = level.config.layout.coordinates_to_position(x, y);
            base.x = prev_x;
            pos.replace_base(base);
        }
    });
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(HiddenZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.hidden.interval),
        damage: factors.hidden.damage,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: hidden_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .hidden_zombie
                .frames
                .first()
                .expect("empty animation hidden_zombie")
                .clone(),
            cost: factors.hidden.cost,
            cooldown: factors.hidden.cooldown,
            hitbox: factors.hidden.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(HIDDEN_ZOMBIE, creature);
    }
}
