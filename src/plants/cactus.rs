use crate::prelude::*;

pub(super) struct PlantsCactusPlugin;

impl Plugin for PlantsCactusPlugin {
    fn build(&self, app: &mut App) {
        initialize(&cactus_systems);
        initialize(&cactus_after);
        app.init_resource::<SpikeActionQueue>();
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(
            Update,
            (test_spike_hit, send_queue)
                .chain()
                .run_if(when_state!(gaming)),
        );
        *cactus_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_cactus),
            ..Default::default()
        });
        *cactus_after.write().unwrap() = Some(app.register_system(add_spike_use));
    }
}

game_conf!(systems cactus_systems);
game_conf!(shooter CactusShooter);
game_conf!(projectile ProjectileSpike);
game_conf!(system cactus_after, Entity);

fn spawn_cactus(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    shooter: Res<CactusShooter>,
) {
    let creature = map.get(&CACTUS).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.cactus.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        game::Health::from(factors.cactus.health),
        SpriteBundle::default(),
    ));
}
#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct SpikeUseCount(pub usize);

fn add_spike_use(In(entity): In<Entity>, mut commands: Commands) {
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.try_insert(SpikeUseCount::default());
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
struct SpikeActionQueue(Vec<game::ProjectileAction>);

fn test_spike_hit(
    mut action: EventReader<game::ProjectileAction>,
    mut q_spike: Query<&mut SpikeUseCount>,
    mut queue: ResMut<SpikeActionQueue>,
    factors: Res<plants::PlantFactors>,
) {
    action.read().for_each(|action| {
        if let game::ProjectileAction::Damage(proj, _entity) = action {
            if let Ok(mut spike) = q_spike.get_mut(*proj) {
                spike.0 += 1;
                if spike.0 >= factors.cactus.pierce {
                    queue.push(game::ProjectileAction::Consumed(*proj));
                }
            }
        }
    })
}

fn send_queue(
    mut queue: ResMut<SpikeActionQueue>,
    mut action_w: EventWriter<game::ProjectileAction>,
) {
    for action in queue.drain(..) {
        action_w.send(action);
    }
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    let spike = Arc::new(game::ProjectileShared {
        anim: plants.spike.clone(),
        hitbox: factors.cactus.spike_box,
    });
    commands.insert_resource(ProjectileSpike(spike.clone()));
    {
        commands.insert_resource(CactusShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_secs_f32(factors.cactus.interval),
            velocity: factors.cactus.velocity.into(),
            proj: game::Projectile {
                damage: factors.cactus.damage,
                manual_consume: true,
                ..Default::default()
            },
            times: factors.cactus.times,
            require_zombie: compn::RequireZombie::InRange,
            shared: spike.clone(),
            after: cactus_after.read().unwrap().unwrap(),
            ..Default::default()
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: cactus_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .cactus
                .frames
                .first()
                .expect("Empty animation cactus")
                .clone(),
            cost: factors.cactus.cost,
            cooldown: factors.cactus.cooldown,
            hitbox: factors.cactus.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(CACTUS, creature);
    }
}
