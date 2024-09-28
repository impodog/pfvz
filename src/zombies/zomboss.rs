use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ZombiesZombossPlugin;

impl Plugin for ZombiesZombossPlugin {
    fn build(&self, app: &mut App) {
        initialize(&zomboss_systems);
        app.init_resource::<ZombossConfig>();
        app.add_systems(PostStartup, (init_config,));
        *zomboss_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_zomboss),
            ..Default::default()
        });
    }
}

game_conf!(systems zomboss_systems);

#[derive(Serialize, Deserialize, Default, Resource, Debug, Clone)]
pub struct ZombossConfig {
    pub health: u32,
}

fn spawn_zomboss(
    In(_pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    config: Res<ZombossConfig>,
    level: Res<level::Level>,
) {
    let (x, y) = level.config.layout.size();
    let pos = level.config.layout.coordinates_to_position(x - 1, y / 2);
    let creature = map.get(&ZOMBOSS).unwrap();
    let pos = pos
        + game::Position::new_xyz(
            0.0,
            0.0,
            factors.zomboss.body_box.height + factors.zomboss.legs_box.height / 2.0,
        );
    let head = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            game::LogicPosition::from_base(pos),
            sprite::Animation::new(zombies.zomboss_head.clone()),
            factors.zomboss.head_box,
            game::Health::from(config.health),
            SpriteBundle::default(),
        ))
        .id();
    let body_pos = game::Position::new(
        factors.zomboss.body_box.width / 2.0,
        0.0,
        -factors.zomboss.body_box.height / 2.0,
        0.0,
    );
    commands
        .spawn((
            game::RelativePosition(body_pos),
            game::Position::default(),
            sprite::Animation::new(zombies.zomboss_body.clone()),
            factors.zomboss.body_box,
            SpriteBundle::default(),
        ))
        .set_parent(head);
    let legs_pos = body_pos
        + game::Position::new(
            0.0,
            0.0,
            -(factors.zomboss.body_box.height + factors.zomboss.legs_box.height) / 2.0 + 1.0,
            0.0,
        );
    commands
        .spawn((
            game::RelativePosition(legs_pos),
            game::Position::default(),
            sprite::Animation::new(zombies.zomboss_legs.clone()),
            factors.zomboss.legs_box,
            SpriteBundle::default(),
        ))
        .set_parent(head);
}

fn init_config(
    mut _commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: ZOMBOSS,
            systems: zomboss_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .zomboss_head
                .frames
                .first()
                .expect("empty animation zomboss_head")
                .clone(),
            cost: factors.zomboss.cost,
            cooldown: factors.zomboss.cooldown,
            hitbox: game::HitBox::default(),
            flags: level::CreatureFlags::GROUND_AQUATIC_ZOMBIE,
        }));
        map.insert(creature);
    }
}
