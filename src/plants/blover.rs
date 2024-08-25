use crate::prelude::*;

pub(super) struct PlantsBloverPlugin;

impl Plugin for PlantsBloverPlugin {
    fn build(&self, app: &mut App) {
        initialize(&blover_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (blover_work,).run_if(when_state!(gaming)));
        *blover_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_blover),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems blover_systems);

#[derive(Component, Debug, Clone, Deref, DerefMut)]
struct BloverTimer(Timer);

fn spawn_blover(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
    mut fog: ResMut<modes::RemoveFogTimer>,
    mut e_fog: EventWriter<modes::RecalculateFogByTimer>,
) {
    let creature = map.get(&BLOVER).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.blover.clone()),
        creature.hitbox,
        BloverTimer(Timer::new(
            Duration::from_secs_f32(factors.blover.duration),
            TimerMode::Once,
        )),
        game::Health::from(factors.blover.health),
        SpriteBundle::default(),
    ));
    let duration = fog
        .duration()
        .max(Duration::from_secs_f32(factors.blover.fog_duration));
    fog.reset();
    fog.set_duration(duration);
    e_fog.send(modes::RecalculateFogByTimer);
}

fn blover_work(
    mut action: EventWriter<game::CreatureAction>,
    mut q_blover: Query<(Entity, &game::Overlay, &mut BloverTimer)>,
    mut q_zombie: Query<
        (
            &game::Position,
            &game::HitBox,
            &mut game::LogicPosition,
            Option<&game::Gravity>,
        ),
        With<game::Zombie>,
    >,
    time: Res<config::FrameTime>,
    factors: Res<plants::PlantFactors>,
    level: Res<level::Level>,
) {
    let bound = level.config.layout.half_size_f32().0;
    q_blover
        .iter_mut()
        .for_each(|(entity, overlay, mut timer)| {
            timer.tick(overlay.delta());
            if timer.just_finished() {
                action.send(game::CreatureAction::Die(entity));
            } else {
                let factor = overlay.factor() * time.diff();
                q_zombie
                    .iter_mut()
                    .for_each(|(pos, hitbox, mut logic, gravity)| {
                        let (x, y) = level
                            .config
                            .layout
                            .position_3d_to_coordinates(logic.base_raw());
                        let disp = pos.z - level.config.layout.get_disp(x);

                        let base_factor = disp.max(0.0) * 2.0;
                        let base_factor = if gravity.is_some() {
                            base_factor * 0.25
                        } else {
                            base_factor * base_factor
                        };
                        let factor = factor * base_factor * factors.blover.velocity_factor;
                        logic.plus_assign(game::Position::new_xy(factor, 0.0));
                        if disp <= 0.5 {
                            let half_width = hitbox.width / 2.0;
                            if logic.base_raw_mut().x - half_width > bound {
                                logic.base_raw_mut().x = bound + half_width;
                            }
                        }
                    });
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
            systems: blover_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .blover
                .frames
                .first()
                .expect("Empty animation blover")
                .clone(),
            cost: factors.blover.cost,
            cooldown: factors.blover.cooldown,
            hitbox: factors.blover.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(BLOVER, creature);
    }
}
