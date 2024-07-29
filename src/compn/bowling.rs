use crate::prelude::*;

pub(super) struct CompnBowlingPlugin;

impl Plugin for CompnBowlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_bowling_impl, bowling_work).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Bowling {
    pub damage: u32,
    pub velocity_y: f32,
}

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
struct BowlingImpl {
    touched: BTreeSet<Entity>,
}

fn add_bowling_impl(mut commands: Commands, q_bowling: Query<Entity, Added<Bowling>>) {
    q_bowling.iter().for_each(|entity| {
        commands.entity(entity).insert(BowlingImpl::default());
    });
}

fn bowling_work(
    mut q_bowling: Query<(
        Entity,
        &mut game::Velocity,
        &mut game::Position,
        &Bowling,
        &mut BowlingImpl,
    )>,
    collision: Res<game::Collision>,
    q_zombie: Query<(), With<game::Zombie>>,
    mut action: EventWriter<game::CreatureAction>,
    config: Res<config::Config>,
    level: Res<level::Level>,
) {
    q_bowling.iter_mut().for_each(
        |(entity, mut velocity, mut pos, bowling, mut bowling_impl)| {
            if let Some(coll) = collision.get(&entity) {
                let mut flag = false;
                for zombie in coll.iter() {
                    if !bowling_impl.contains(zombie) && q_zombie.get(*zombie).is_ok() {
                        flag = true;
                        bowling_impl.insert(*zombie);
                        action.send(game::CreatureAction::Damage(
                            *zombie,
                            multiply_uf!(bowling.damage, config.gamerule.damage.0),
                        ));
                        break;
                    }
                }
                if flag {
                    if velocity.y == 0.0 {
                        velocity.y = if rand::thread_rng().gen_ratio(1, 2) {
                            bowling.velocity_y
                        } else {
                            -bowling.velocity_y
                        };
                    } else {
                        velocity.y = -velocity.y;
                    }
                }
            }
            let hsize = level.config.layout.half_size_f32();
            if pos.y > hsize.1 {
                pos.y = hsize.1;
                velocity.y = -velocity.y.abs();
            } else if pos.y < -hsize.1 {
                pos.y = -hsize.1;
                velocity.y = velocity.y.abs();
            }
        },
    );
}
