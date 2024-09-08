use crate::prelude::*;

pub(super) struct ZombiesDiggerPlugin;

impl Plugin for ZombiesDiggerPlugin {
    fn build(&self, app: &mut App) {
        initialize(&digger_zombie_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (digger_work,).run_if(when_state!(gaming)));
        app.init_resource::<DiggerPleaseGoUp>();
        *digger_zombie_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_digger_zombie),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems digger_zombie_systems);
game_conf!(walker DiggerZombieWalker);
game_conf!(breaks HardCapBreaks);

#[derive(Component, Debug)]
pub enum DiggerStatus {
    Start(Timer),
    Inside,
    Outside,
}
impl Default for DiggerStatus {
    fn default() -> Self {
        Self::Start(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

fn spawn_digger_zombie(
    In(pos): In<game::LogicPosition>,
    zombies: Res<assets::SpriteZombies>,
    mut commands: Commands,
    factors: Res<zombies::ZombieFactors>,
    map: Res<game::CreatureMap>,
    breaks: Res<HardCapBreaks>,
) {
    let creature = map.get(&DIGGER_ZOMBIE).unwrap();
    let entity = commands
        .spawn((
            game::Zombie,
            creature.clone(),
            pos,
            game::Velocity::from(factors.digger.dig_velocity),
            sprite::Animation::new(zombies.digger.clone()),
            compn::Dying::new(zombies.digger_dying.clone()),
            creature.hitbox,
            DiggerStatus::default(),
            // Yes, diggers are magnetic by themselves
            game::Magnetic,
            game::Health::from(factors.digger.self_health),
            SpriteBundle::default(),
        ))
        .id();
    commands
        .spawn((
            game::Position::default(),
            game::RelativePosition::new(0.03, 0.0, 0.4, 0.0),
            factors.digger.hard_cap_box,
            sprite::Animation::new(zombies.hard_cap.clone()),
            game::Armor::new(factors.digger.hard_cap_health),
            compn::Breaks(breaks.0.clone()),
            game::LayerDisp(0.01),
            SpriteBundle::default(),
        ))
        .set_parent(entity);
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct DiggerPleaseGoUp(pub BTreeSet<Entity>);

fn digger_work(
    commands: ParallelCommands,
    mut q_digger: Query<(
        Entity,
        &game::Overlay,
        &mut game::SizeCrop,
        &game::Position,
        &mut game::HitBox,
        &mut game::Velocity,
        &mut DiggerStatus,
        &mut sprite::Animation,
    )>,
    factors: Res<zombies::ZombieFactors>,
    level: Res<level::Level>,
    zombies: Res<assets::SpriteZombies>,
    walker: Res<DiggerZombieWalker>,
    go_up: ResMut<DiggerPleaseGoUp>,
) {
    let stretch_factor = factors.digger.underground_box.height / factors.digger.self_box.height;
    let left_bound = factors.digger.self_box.width / 2.0 - level.config.layout.half_size_f32().0;
    let go_up = Mutex::new(go_up);
    q_digger.par_iter_mut().for_each(
        |(entity, overlay, mut size, pos, mut hitbox, mut velocity, mut status, mut anim)| {
            match &mut *status {
                DiggerStatus::Start(ref mut timer) => {
                    timer.tick(overlay.delta());
                    if timer.just_finished() {
                        *status = DiggerStatus::Inside;
                        *hitbox = factors.digger.underground_box;
                        size.y_crop.multiply(stretch_factor);
                        size.y_stretch.multiply(1.0 / stretch_factor);
                    }
                }
                DiggerStatus::Inside => {
                    let flag = {
                        let mut go_up = go_up.lock().unwrap();
                        if go_up.contains(&entity) {
                            go_up.remove(&entity);
                            true
                        } else {
                            false
                        }
                    };
                    if flag || pos.x <= left_bound {
                        *status = DiggerStatus::Outside;
                        *hitbox = factors.digger.self_box;
                        *velocity = factors.digger.velocity.into();
                        if flag {
                            velocity.x = -velocity.x.abs();
                        } else {
                            anim.replace(zombies.digger_mirror.clone());
                        }
                        size.y_crop.divide(stretch_factor);
                        size.y_stretch.divide(1.0 / stretch_factor);
                        commands.command_scope(|mut commands| {
                            commands
                                .entity(entity)
                                .try_insert(compn::Walker(walker.0.clone()))
                                .remove::<game::Magnetic>();
                        })
                    }
                }
                DiggerStatus::Outside => {}
            }
        },
    );
}

fn init_config(
    mut commands: Commands,
    zombies: Res<assets::SpriteZombies>,
    factors: Res<zombies::ZombieFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(DiggerZombieWalker(Arc::new(compn::WalkerShared {
        interval: Duration::from_secs_f32(factors.digger.interval),
        damage: factors.digger.damage,
    })));
    commands.insert_resource(HardCapBreaks(Arc::new(compn::BreaksShared {
        v: vec![zombies.hard_cap.clone(), zombies.hard_cap_damaged.clone()],
        init: factors.digger.hard_cap_health,
    })));
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: digger_zombie_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: zombies
                .digger
                .frames
                .first()
                .expect("empty animation digger")
                .clone(),
            cost: factors.digger.cost,
            cooldown: factors.digger.cooldown,
            hitbox: factors.digger.self_box,
            flags: level::CreatureFlags::GROUND_ZOMBIE,
        }));
        map.insert(DIGGER_ZOMBIE, creature);
    }
}
