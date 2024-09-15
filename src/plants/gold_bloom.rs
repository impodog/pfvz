use crate::prelude::*;

pub(super) struct PlantsGoldBloomPlugin;

impl Plugin for PlantsGoldBloomPlugin {
    fn build(&self, app: &mut App) {
        initialize(&gold_bloom_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (gold_bloom_work,));
        *gold_bloom_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_gold_bloom),
            ..Default::default()
        });
    }
}

game_conf!(systems gold_bloom_systems);

#[derive(Component, Deref, DerefMut, Debug, Default)]
pub struct GoldBloomTimer {
    #[deref]
    pub timer: Timer,
    pub count: usize,
}

fn spawn_gold_bloom(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&GOLD_BLOOM).unwrap();
    let mut timer = Timer::from_seconds(factors.gold_bloom.interval, TimerMode::Repeating);
    timer.set_elapsed(timer.duration());
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.gold_bloom.clone()),
        creature.hitbox,
        GoldBloomTimer {
            timer,
            ..Default::default()
        },
        game::Health::from(factors.gold_bloom.health),
        SpriteBundle::default(),
    ));
}

fn gold_bloom_work(
    commands: ParallelCommands,
    action: EventWriter<game::CreatureAction>,
    mut q_gold_bloom: Query<(Entity, &game::Overlay, &mut GoldBloomTimer, &game::Position)>,
    factors: Res<plants::PlantFactors>,
) {
    let action = Mutex::new(action);
    q_gold_bloom
        .par_iter_mut()
        .for_each(|(entity, overlay, mut timer, pos)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                commands.command_scope(|mut commands| {
                    commands.spawn((
                        *pos,
                        game::Velocity::from(factors.gold_bloom.velocity),
                        collectible::Collectible::Sun(factors.gold_bloom.multiplier),
                    ));
                });
                timer.count += 1;
                if timer.count >= factors.gold_bloom.times {
                    action
                        .lock()
                        .unwrap()
                        .send(game::CreatureAction::Die(entity));
                }
            }
        });
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: gold_bloom_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .gold_bloom
                .frames
                .first()
                .expect("Empty animation gold_bloom")
                .clone(),
            cost: factors.gold_bloom.cost,
            cooldown: factors.gold_bloom.cooldown,
            hitbox: factors.gold_bloom.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(GOLD_BLOOM, creature);
    }
}
