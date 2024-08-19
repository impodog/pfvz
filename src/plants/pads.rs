use crate::prelude::*;

pub(super) struct PlantsPadsPlugin;

impl Plugin for PlantsPadsPlugin {
    fn build(&self, app: &mut App) {
        initialize(&lily_pad_systems);
        initialize(&flower_pot_systems);
        app.add_systems(PostStartup, (init_config,));
        *lily_pad_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_lily_pad),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
        *flower_pot_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_flower_pot),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems lily_pad_systems);
game_conf!(systems flower_pot_systems);

fn spawn_lily_pad(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&LILY_PAD).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.lily_pad.clone()),
        creature.hitbox,
        game::Health::from(factors.lily_pad.health),
        SpriteBundle::default(),
    ));
}

fn spawn_flower_pot(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&FLOWER_POT).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.flower_pot.clone()),
        creature.hitbox,
        game::Health::from(factors.flower_pot.health),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut _commands: Commands,
    plants: Res<assets::SpritePlants>,
    factors: Res<plants::PlantFactors>,
    mut map: ResMut<game::CreatureMap>,
) {
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: lily_pad_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .lily_pad
                .frames
                .first()
                .expect("Empty animation lily_pad")
                .clone(),
            cost: factors.lily_pad.cost,
            cooldown: factors.lily_pad.cooldown,
            hitbox: factors.lily_pad.self_box,
            flags: level::CreatureFlags::LILY_PAD,
        }));
        map.insert(LILY_PAD, creature);
    }
    {
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: flower_pot_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .flower_pot
                .frames
                .first()
                .expect("Empty animation flower_pot")
                .clone(),
            cost: factors.flower_pot.cost,
            cooldown: factors.flower_pot.cooldown,
            hitbox: factors.flower_pot.self_box,
            flags: level::CreatureFlags::FLOWER_POT,
        }));
        map.insert(FLOWER_POT, creature);
    }
}
