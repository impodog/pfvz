use crate::prelude::*;

pub(super) struct PlantsPeaPlugin;

impl Plugin for PlantsPeaPlugin {
    fn build(&self, app: &mut App) {
        initialize(&peashooter_systems);
        app.add_systems(PostStartup, (init_config,));
        *peashooter_systems.write().unwrap() = Some(game::CreatureSystems {
            spawn: app.register_system(spawn_peashooter),
            die: app.register_system(plants::default::die),
            damage: app.register_system(plants::default::damage),
        });
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            |mut action: EventWriter<game::CreatureAction>, mut b: Local<bool>| {
                if !*b {
                    *b = true;
                    action.send(game::CreatureAction::Spawn(
                        PEASHOOTER,
                        game::Position::new_xy(1.0, 1.0),
                    ));
                }
            },
        );
    }
}

game_conf!(projectile ProjectilePea);
game_conf!(shooter PeashooterShooter);
game_conf!(systems peashooter_systems);

fn spawn_peashooter(
    In(pos): In<game::Position>,
    mut commands: Commands,
    map: Res<game::CreatureMap>,
    shooter: Res<PeashooterShooter>,
) {
    let creature = map.get(&PEASHOOTER).unwrap();
    commands.spawn((
        pos,
        sprite::Animation::new(creature.anim.clone()),
        creature.hitbox,
        compn::Shooter(shooter.0.clone()),
        SpriteBundle::default(),
    ));
}

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    mut map: ResMut<game::CreatureMap>,
) {
    let pea = Arc::new(game::ProjectileShared {
        anim: plants.pea.clone(),
        hitbox: game::HitBox::new(0.5, 0.5),
    });
    commands.insert_resource(ProjectilePea(pea.clone()));
    {
        commands.insert_resource(PeashooterShooter(Arc::new(compn::ShooterShared {
            interval: Duration::from_millis(2000),
            vel: game::Velocity::new(0.05, 0.0, 0.0, 2.0),
            proj: game::Projectile {
                damage: 20,
                instant: true,
            },
            shared: pea.clone(),
        })));
        let creature = game::Creature(Arc::new(game::CreatureShared {
            systems: peashooter_systems
                .read()
                .unwrap()
                .expect("systems are not initialized"),
            anim: plants.peashooter.clone(),
            cost: 100,
            hitbox: game::HitBox::new(1.0, 1.0),
        }));
        map.insert(PEASHOOTER, creature);
    }
}
