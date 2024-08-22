use crate::prelude::*;

pub(super) struct PlantsFirePlugin;

impl Plugin for PlantsFirePlugin {
    fn build(&self, app: &mut App) {
        initialize(&torchwood_systems);
        app.add_systems(PostStartup, (init_config,));
        app.add_systems(Update, (torchwood_ignite,));
        *torchwood_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_torchwood),
            die: app.register_system(compn::default::die),
            damage: app.register_system(compn::default::damage),
        });
    }
}

game_conf!(systems torchwood_systems);

#[derive(Component)]
pub struct TorchwoodMarker;

fn spawn_torchwood(
    In(pos): In<game::LogicPosition>,
    mut commands: Commands,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
    map: Res<game::CreatureMap>,
) {
    let creature = map.get(&TORCHWOOD).unwrap();
    commands.spawn((
        game::Plant,
        creature.clone(),
        pos,
        sprite::Animation::new(plants.torchwood.clone()),
        creature.hitbox,
        TorchwoodMarker,
        modes::RemoveFog(factors.torchwood.light_range.into()),
        game::Health::from(factors.torchwood.health),
        SpriteBundle::default(),
    ));
}

fn torchwood_ignite(
    commands: ParallelCommands,
    q_torchwood: Query<Entity, With<TorchwoodMarker>>,
    q_proj: Query<
        (&game::HitBox, &game::Projectile),
        (With<game::Projectile>, Without<compn::FireProjectile>),
    >,
    collision: Res<game::Collision>,
    factors: Res<plants::PlantFactors>,
    plants: Res<assets::SpritePlants>,
) {
    q_torchwood.par_iter().for_each(|entity| {
        if let Some(coll) = collision.get(&entity) {
            for proj in coll.iter() {
                if let Ok((hitbox, projectile)) = q_proj.get(*proj) {
                    if projectile.area {
                        continue;
                    }
                    commands.command_scope(|mut commands| {
                        commands
                            .entity(*proj)
                            .insert(compn::FireProjectile::from(factors.torchwood.fire));
                        commands
                            .spawn((
                                game::Position::default(),
                                game::LayerDisp(0.1),
                                *hitbox,
                                sprite::Animation::new(plants.fire.clone()),
                                SpriteBundle::default(),
                            ))
                            .set_parent(*proj);
                    });
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
            systems: torchwood_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            image: plants
                .torchwood
                .frames
                .first()
                .expect("Empty animation torchwood")
                .clone(),
            cost: factors.torchwood.cost,
            cooldown: factors.torchwood.cooldown,
            hitbox: factors.torchwood.self_box,
            flags: level::CreatureFlags::TERRESTRIAL_PLANT,
        }));
        map.insert(TORCHWOOD, creature);
    }
}
