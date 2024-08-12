use crate::prelude::*;

pub(super) struct ModesWhackPlugin;

impl Plugin for ModesWhackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (spawn_whack,));
        app.add_systems(
            Update,
            (spawn_grave, whack_work, whack_move, move_to_grave).run_if(when_state!(gaming)),
        );
        app.init_resource::<WhackTimer>();
    }
}

#[derive(Component)]
pub struct WhackMarker;

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct WhackTimer(pub Timer);

fn spawn_whack(
    mut commands: Commands,
    level: Res<level::Level>,
    factors: Res<collectible::ItemFactors>,
    items: Res<assets::SpriteItems>,
) {
    if level.config.game == level::GameKind::WhackAZombie {
        commands.spawn((
            WhackMarker,
            game::Position::default(),
            factors.whack.self_box,
            sprite::Animation::new(items.whack.clone()),
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            },
        ));
        for y in 0..5 {
            let pos = level.config.layout.coordinates_to_position(0, y);
            let pos = game::LogicPosition::from_base(pos);
            commands.run_system_with_input(
                plants::sun_shroom_systems.read().unwrap().unwrap().spawn,
                pos,
            );
            for x in 1..=2 {
                let pos = level.config.layout.coordinates_to_position(x, y);
                let pos = game::LogicPosition::from_base(pos);
                commands.run_system_with_input(
                    plants::iceberg_lettuce_systems
                        .read()
                        .unwrap()
                        .unwrap()
                        .spawn,
                    pos,
                );
            }
        }
        commands.insert_resource(WhackTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )));
    }
}

fn move_to_grave(
    mut q_zombie: Query<&mut game::LogicPosition, Added<game::Zombie>>,
    q_grave: Query<(&game::Position, &game::Creature), (With<game::Plant>, Without<game::Zombie>)>,
    level: Res<level::Level>,
) {
    if level.config.game == level::GameKind::WhackAZombie {
        let mut graves = Vec::new();
        q_grave.iter().for_each(|(pos, creature)| {
            if creature.flags == level::CreatureFlags::GRAVE {
                graves.push(*pos);
            }
        });
        if !graves.is_empty() {
            q_zombie.par_iter_mut().for_each(|mut pos| {
                let regular = level.config.layout.regularize(pos.base);
                let grave = graves.choose(&mut rand::thread_rng()).unwrap();
                pos.x += grave.x - regular.x;
                pos.y += grave.y - regular.y;
            });
        }
    }
}

fn spawn_grave(
    mut commands: Commands,
    mut timer: ResMut<WhackTimer>,
    factors: Res<collectible::ItemFactors>,
    time: Res<config::FrameTime>,
    level: Res<level::Level>,
) {
    if level.config.game == level::GameKind::WhackAZombie {
        timer.tick(time.delta());
        if timer.just_finished() {
            timer.set_duration(Duration::from(factors.whack.interval));
            commands.run_system(plants::grave_spawn_anywhere.read().unwrap().unwrap());
        }
    }
}

fn whack_work(
    q_whack: Query<Entity, With<WhackMarker>>,
    mut action: EventWriter<game::CreatureAction>,
    cursor: Res<info::CursorInfo>,
    collision: Res<game::Collision>,
    factors: Res<collectible::ItemFactors>,
    q_zombie: Query<(), With<game::Zombie>>,
    audio: Res<Audio>,
    items: Res<assets::AudioItems>,
) {
    q_whack.iter().for_each(|entity| {
        if cursor.left {
            if let Some(coll) = collision.get(&entity) {
                for zombie in coll.iter() {
                    if q_zombie.get(*zombie).is_ok() {
                        // NOTE: Multiply the damage multiplier?
                        action.send(game::CreatureAction::Damage(*zombie, factors.whack.damage));
                        audio.play(items.whack.random());
                        break;
                    }
                }
            }
        }
    });
}

fn whack_move(
    mut q_whack: Query<&mut game::Position, With<WhackMarker>>,
    cursor: Res<info::CursorInfo>,
) {
    q_whack.iter_mut().for_each(|mut pos| {
        *pos = cursor.pos;
    });
}
