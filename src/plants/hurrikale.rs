use crate::prelude::*;

pub(super) struct PlantsHurrikalePlugin;

impl Plugin for PlantsHurrikalePlugin {
    fn build(&self, app: &mut App) {
        initialize(&hurrikale_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (hurrikale_work,).run_if(when_state!(gaming)));
        *hurrikale_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_hurrikale),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems hurrikale_systems);

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct HurrikaleTimer {
    #[deref]
    timer: Timer,
    target: Vec<Entity>,
}

fn spawn_hurrikale(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    q_zombie: Query<(Entity, &game::Position), With<game::Zombie>>,
    level: Res<level::Level>,
) {
    let creature = map.get(&HURRIKALE).unwrap();
    let target = q_zombie
        .iter()
        .filter_map(|(entity, zombie_pos)| {
            if level.config.layout.same_y(pos.base_raw(), zombie_pos) {
                Some(entity)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.hurrikale.clone()),
        creature.hitbox,
        HurrikaleTimer {
            timer: Timer::new(
                Duration::from_secs_f32(factors.hurrikale.interval),
                TimerMode::Once,
            ),
            target,
        },
        game::Health::from(factors.hurrikale.health),
        SpriteBundle::default(),
    ));
}

fn hurrikale_work(
    mut action: EventWriter<game::CreatureAction>,
    mut q_hurrikale: Query<(Entity, &game::Overlay, &mut HurrikaleTimer)>,
    mut q_zombie: Query<&mut game::LogicPosition>,
    time: Res<config::FrameTime>,
    factors: Res<plants::PlantFactors>,
    level: Res<level::Level>,
) {
    q_hurrikale
        .iter_mut()
        .for_each(|(entity, overlay, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                action.send(game::CreatureAction::Die(entity));
            } else {
                let factor = overlay.factor() * time.diff();
                timer.target.iter().for_each(|zombie| {
                    if let Ok(mut logic) = q_zombie.get_mut(*zombie) {
                        logic.plus_assign(game::Position::new_xy(
                            factors.hurrikale.blow_velocity * factor,
                            0.0,
                        ));
                        let max = level.config.layout.half_size_f32().0;
                        if logic.base_raw().x > max {
                            logic.base_raw_mut().x = max;
                        }
                    }
                });
            }
        })
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: hurrikale_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .hurrikale
                .frames
                .first()
                .expect("Empty animation hurrikale")
                .clone(),
            cost: factors.hurrikale.cost,
            cooldown: factors.hurrikale.cooldown,
            hitbox: factors.hurrikale.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(HURRIKALE, creature);
    }
}
