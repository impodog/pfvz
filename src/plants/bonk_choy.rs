use crate::prelude::*;

pub(super) struct PlantsBonkChoyPlugin;

impl Plugin for PlantsBonkChoyPlugin {
    fn build(&self, app: &mut App) {
        initialize(&bonk_choy_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (beat_zombie,));
        *bonk_choy_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_bonk_choy),
            ..Default::default()
        });
    }
}

game_conf!(systems bonk_choy_systems);

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct BonkChoyTimer(pub Timer);

fn spawn_bonk_choy(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&BONK_CHOY).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.bonk_choy.clone()),
        creature.hitbox,
        BonkChoyTimer(Timer::new(
            Duration::from_secs_f32(factors.bonk_choy.interval),
            TimerMode::Repeating,
        )),
        game::Health::from(factors.bonk_choy.health),
        SpriteBundle::default(),
    ));
}

fn beat_zombie(
    commands: ParallelCommands,
    action: EventWriter<game::CreatureAction>,
    mut q_bonk_choy: Query<(Entity, &game::Overlay, &game::Position, &mut BonkChoyTimer)>,
    q_zombie: Query<(Entity, &game::Position, &game::HitBox), With<game::Zombie>>,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    config: Res<config::Config>,
) {
    let action = Mutex::new(action);
    q_bonk_choy
        .par_iter_mut()
        .for_each(|(entity, overlay, pos, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                let range = game::PositionRange::from(factors.bonk_choy.range) + *pos;
                for (zombie, zombie_pos, zombie_hitbox) in q_zombie.iter() {
                    if range.contains(zombie_pos, zombie_hitbox) {
                        action.lock().unwrap().send(game::CreatureAction::Damage(
                            zombie,
                            multiply_uf!(factors.bonk_choy.damage, config.gamerule.damage.0),
                        ));
                        commands.command_scope(|mut commands| {
                            commands.entity(entity).try_insert(compn::AnimationThenDo {
                                anim: if zombie_pos.x > pos.x {
                                    plants.bonk_choy_right.clone()
                                } else {
                                    plants.bonk_choy_left.clone()
                                },
                                ..Default::default()
                            });
                        });
                        break;
                    }
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
            id: BONK_CHOY,
            systems: bonk_choy_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .bonk_choy
                .frames
                .first()
                .expect("Empty animation bonk_choy")
                .clone(),
            cost: factors.bonk_choy.cost,
            cooldown: factors.bonk_choy.cooldown,
            hitbox: factors.bonk_choy.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
