use crate::prelude::*;

pub(super) struct ZombiesJitbPlugin;

impl Plugin for ZombiesJitbPlugin {
    fn build(&self, app: &mut App) {
        initialize(&jitb_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (jitb_trigger,).run_if(when_state!(gaming)));
        *jitb_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_jitb_zombie),
            die: app.register_system(jitb_zombie_die),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
        *jitb_zombie_explode.write().unwrap() = Some(app.register_system(jitb_explode));
    }
}

game_conf!(systems jitb_zombie_systems);
game_conf!(system jitb_zombie_explode, Entity);

#[derive(Component, Default)]
pub struct JitbZombieStatus(pub bool);

#[derive(Component, Debug, Clone)]
pub struct JitbExplode(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct JitbAudioHandle(pub Handle<AudioInstance>);

fn spawn_jitb_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    audio: Res<Audio>,
    audio_zombies: Res<assets::AudioZombies>,
) {
    let creature = map.get(&JITB_ZOMBIE).unwrap();
    let handle = audio.play(audio_zombies.jitb.random()).handle();
    commands.spawn((
        game::Zombie,
        creature.clone(),
        pos,
        game::Velocity::from(factors.jitb.velocity),
        sprite::Animation::new(zombies.jitb_zombie.clone()),
        compn::Dying::new(zombies.jitb_zombie_dying.clone()),
        creature.hitbox,
        JitbZombieStatus::default(),
        JitbAudioHandle(handle),
        game::Health::from(factors.jitb.self_health),
        SpriteBundle::default(),
    ));
}

fn jitb_zombie_die(
    In(entity): In<Entity>,
    mut commands: Commands,
    q_jitb: Query<&JitbAudioHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Ok(handle) = q_jitb.get(entity) {
        if let Some(instance) = audio_instances.get_mut(handle.id()) {
            instance.stop(AudioTween::default());
        }
    }
    if let Some(commands) = commands.get_entity(entity) {
        commands.despawn_recursive();
    }
}

fn jitb_explode(
    In(entity): In<Entity>,
    mut commands: Commands,
    mut q_jitb: Query<(&game::Position, &game::Health, &mut JitbExplode)>,
    mut action: EventWriter<game::CreatureAction>,
    factors: Res<zombies::ZombieFactors>,
    plants: Res<assets::SpritePlants>,
    audio: Res<Audio>,
    audio_zombies: Res<assets::AudioZombies>,
) {
    if let Ok((pos, health, mut explode)) = q_jitb.get_mut(entity) {
        if !health.is_dying() {
            for target in explode.0.drain(..) {
                action.send(game::CreatureAction::Damage(target, factors.jitb.damage));
            }
            action.send(game::CreatureAction::Die(entity));
            commands.spawn((
                sprite::Animation::new(plants.boom.clone()),
                *pos,
                game::HitBox::new(factors.jitb.range.x.1 * 2.0, factors.jitb.range.y.1 * 2.0),
                level::Banner::new(Duration::from_secs_f32(factors.jitb.animation_time)),
                SpriteBundle::default(),
            ));
            audio.play(audio_zombies.explode.random());
        }
    }
}

fn jitb_trigger(
    commands: ParallelCommands,
    mut q_jitb: Query<(
        Entity,
        &game::Health,
        &game::Position,
        &mut JitbZombieStatus,
    )>,
    q_pos: Query<(&game::Position, &game::HitBox)>,
    q_plant: Query<Entity, With<game::Plant>>,
    q_zombie: Query<Entity, With<game::Zombie>>,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
) {
    q_jitb
        .par_iter_mut()
        .for_each(|(entity, health, pos, mut jitb)| {
            if !jitb.0 && !health.is_dying() {
                let range = game::PositionRange::from(factors.jitb.range) + *pos;
                let explode = if q_zombie.get(entity).is_ok() {
                    q_plant
                        .iter()
                        .filter(|entity| {
                            q_pos
                                .get(*entity)
                                .is_ok_and(|(pos, hitbox)| range.contains(pos, hitbox))
                        })
                        .collect::<Vec<_>>()
                } else {
                    q_zombie
                        .iter()
                        .filter(|entity| {
                            q_pos
                                .get(*entity)
                                .is_ok_and(|(pos, hitbox)| range.contains(pos, hitbox))
                        })
                        .collect::<Vec<_>>()
                };
                if !explode.is_empty() {
                    jitb.0 = true;
                    commands.command_scope(|mut commands| {
                        commands
                            .entity(entity)
                            .try_insert(JitbExplode(explode))
                            .try_insert(compn::AnimationThenDo {
                                anim: zombies.jitb_zombie_explode.clone(),
                                work: jitb_zombie_explode.read().unwrap().unwrap(),
                            });
                    });
                }
            }
        })
}

fn init_config(
    mut _commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            id: JITB_ZOMBIE,
            systems: jitb_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .jitb_zombie
                .frames
                .first()
                .expect("empty animation jitb_zombie")
                .clone(),
            cost: factors.jitb.cost,
            cooldown: factors.jitb.cooldown,
            hitbox: factors.jitb.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(creature);
    }
}
