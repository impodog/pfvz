use crate::prelude::*;

pub(super) struct PlantsSpikePlugin;

impl Plugin for PlantsSpikePlugin {
    fn build(&self, app: &mut App) {
        initialize(&spikeweed_systems);
        initialize(&spikeweed_contact_many);
        app.add_systems(PostStartup, (init_config,));
        *spikeweed_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_spikeweed),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        });
        *spikeweed_contact_many.write().unwrap() = Some(app.register_system(spikeweed_work));
    }
}

game_conf!(systems spikeweed_systems);
game_conf!(system spikeweed_contact_many, (Entity, compn::EntityList));

fn spawn_spikeweed(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&SPIKEWEED).unwrap();
    commands.spawn((
        game::Plant,
        compn::WalkerImmunity,
        creature.clone(),
        pos.plus(game::Position::new_xyz(0.0, 0.0, 0.2)),
        sprite::Animation::new(plants.spikeweed.clone()),
        creature.hitbox,
        compn::ContactMany {
            system: spikeweed_contact_many.read().unwrap().unwrap(),
            interval: Duration::from_secs_f32(factors.spikeweed.interval),
        },
        game::Health::from(factors.spikeweed.health),
        SpriteBundle::default(),
    ));
}

fn spikeweed_work(
    In((entity, enemies)): In<(Entity, compn::EntityList)>,
    mut commands: Commands,
    mut action: EventWriter<game::CreatureAction>,
    q_zomboni: Query<&game::Health, With<zombies::ZomboniMarker>>,
    factors: Res<plants::PlantFactors>,
    audio: Res<Audio>,
    audio_plants: Res<assets::AudioPlants>,
) {
    let mut despawn = false;
    for enemy in enemies.iter() {
        let damage = if let Ok(health) = q_zomboni.get(*enemy) {
            despawn = true;
            health.hp
        } else {
            factors.spikeweed.damage
        };
        action.send(game::CreatureAction::Damage(*enemy, damage));
        if despawn {
            break;
        }
    }
    audio.play(audio_plants.spikeweed.random());
    if despawn {
        commands.entity(entity).despawn_recursive();
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
            systems: spikeweed_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .spikeweed
                .frames
                .first()
                .expect("Empty animation spikeweed")
                .clone(),
            cost: factors.spikeweed.cost,
            cooldown: factors.spikeweed.cooldown,
            hitbox: factors.spikeweed.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(SPIKEWEED, creature);
    }
}
