use crate::prelude::*;

pub(super) struct PlantsEthylenePlugin;

impl Plugin for PlantsEthylenePlugin {
    fn build(&self, app: &mut App) {
        initialize(&ethylene_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (ethylene_work,).run_if(when_state!(gaming)));
        *ethylene_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_ethylene),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        })
    }
}

game_conf!(systems ethylene_systems);

#[derive(Component, Deref, DerefMut)]
pub struct EthyleneTimer(pub Timer);

fn spawn_ethylene(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&ETHYLENE).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.ethylene.clone()),
        creature.hitbox,
        EthyleneTimer(Timer::from_seconds(
            factors.ethylene.duration,
            TimerMode::Once,
        )),
        game::Health::from(factors.ethylene.health),
        SpriteBundle::default(),
    ));
}

fn ethylene_work(
    mut action: EventWriter<game::CreatureAction>,
    mut q_ethyl: Query<(Entity, &game::Overlay, &mut EthyleneTimer)>,
    factors: Res<plants::PlantFactors>,
    mut cooldown: ResMut<game::SelectionCooldown>,
    selection: Res<game::Selection>,
) {
    let mut sum = Duration::default();
    q_ethyl.iter_mut().for_each(|(entity, overlay, mut timer)| {
        timer.tick(overlay.delta());
        if timer.just_finished() {
            action.send(game::CreatureAction::Die(entity));
        } else {
            sum += overlay.delta();
        }
    });
    if !sum.is_zero() {
        sum = sum.mul_f32(factors.ethylene.factor);
        for (cooldown, id) in cooldown.iter_mut().zip(selection.iter()) {
            if IdFeature::from(*id) == IdFeature::Plant {
                cooldown.tick(sum);
            }
        }
    }
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: ethylene_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .ethylene
                .frames
                .first()
                .expect("Empty animation ethylene")
                .clone(),
            cost: factors.ethylene.cost,
            cooldown: factors.ethylene.cooldown,
            hitbox: factors.ethylene.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(ETHYLENE, creature);
    }
}
