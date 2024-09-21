use crate::prelude::*;

pub(super) struct PlantsPlanternPlugin;

impl Plugin for PlantsPlanternPlugin {
    fn build(&self, app: &mut App) {
        initialize(&plantern_systems);
        app.add_systems(PostStartup, (init_config,));
        *plantern_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_plantern),
            ..Default::default()
        });
    }
}

game_conf!(systems plantern_systems);

fn spawn_plantern(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&PLANTERN).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.plantern.clone()),
        creature.hitbox,
        modes::RemoveFog(factors.plantern.range.into()),
        game::Health::from(factors.plantern.health),
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
            id: PLANTERN,
            systems: plantern_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .plantern
                .frames
                .first()
                .expect("Empty animation plantern")
                .clone(),
            cost: factors.plantern.cost,
            cooldown: factors.plantern.cooldown,
            hitbox: factors.plantern.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(creature);
    }
}
