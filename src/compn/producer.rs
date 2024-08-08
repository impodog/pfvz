use crate::prelude::*;

pub(super) struct CompnProducerPlugin;

impl Plugin for CompnProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (add_producer_impl, producer_work).run_if(when_state!(gaming)),
        );
    }
}

// Anything that uses this shoots projectile of their ally
#[derive(Debug, Clone)]
pub struct ProducerShared {
    pub interval: Duration,
    pub velocity: game::VelocityAny,
    pub collectible: collectible::Collectible,
}
#[derive(Component, Debug, Clone, Deref)]
pub struct Producer(pub Arc<ProducerShared>);

#[derive(Component, Debug, Clone)]
struct ProducerImpl {
    timer: Timer,
}
impl From<&Producer> for ProducerImpl {
    fn from(value: &Producer) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(value.interval.as_secs_f32() / 1.5),
                TimerMode::Repeating,
            ),
        }
    }
}

fn add_producer_impl(
    mut commands: Commands,
    q_producer: Query<(Entity, &Producer), Changed<Producer>>,
) {
    q_producer.iter().for_each(|(entity, producer)| {
        commands
            .entity(entity)
            .try_insert((ProducerImpl::from(producer),));
    });
}

fn producer_work(
    mut commands: Commands,
    mut q_producer: Query<(
        &game::Overlay,
        &game::Position,
        &Producer,
        &mut ProducerImpl,
    )>,
) {
    q_producer
        .iter_mut()
        .for_each(|(overlay, pos, producer, mut producer_impl)| {
            producer_impl.timer.tick(overlay.delta());
            if producer_impl.timer.just_finished() {
                commands.spawn((
                    *pos,
                    game::Velocity::from(producer.velocity),
                    producer.collectible.clone(),
                    game::Gravity,
                ));
                // This adds fun to the sunflower etc. :)
                producer_impl.timer.set_duration(Duration::from_secs_f32(
                    producer.interval.as_secs_f32() * rand::thread_rng().gen_range(0.9..=1.1),
                ));
            }
        });
}
