use crate::prelude::*;

pub(super) struct GameOverlayPlugin;

impl Plugin for GameOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (add_overlay,));
        app.add_systems(PreUpdate, (update_overlay,));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Overlay {
    speed: f32,
    speed_queue: BTreeMap<Orderedf32, usize>,
    delta: Duration,
}
impl Default for Overlay {
    fn default() -> Self {
        Self {
            speed: 1.0,
            speed_queue: Default::default(),
            delta: Default::default(),
        }
    }
}
impl Overlay {
    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn multiply(&mut self, rate: f32) {
        self.speed *= rate;
        match self.speed_queue.entry(rate.into()) {
            std::collections::btree_map::Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
            std::collections::btree_map::Entry::Occupied(mut occupied) => {
                *occupied.get_mut() += 1;
            }
        }
    }

    pub fn divide(&mut self, rate: f32) {
        self.speed /= rate;
        let remove = match self.speed_queue.entry(rate.into()) {
            std::collections::btree_map::Entry::Occupied(mut occupied) => {
                let value = occupied.get().saturating_sub(1);
                *occupied.get_mut() = value;
                value == 0
            }
            _ => false,
        };
        if remove {
            self.speed_queue.remove(&rate.into());
        }
    }
}

fn add_overlay(mut commands: Commands, q_creature: Query<Entity, Added<game::Creature>>) {
    q_creature.iter().for_each(|entity| {
        commands.entity(entity).insert(Overlay::default());
    });
}

fn update_overlay(time: Res<config::FrameTime>, mut q_overlay: Query<&mut Overlay>) {
    q_overlay.par_iter_mut().for_each(|mut overlay| {
        overlay.delta = Duration::from_secs_f32(time.delta().as_secs_f32() * overlay.speed);
    });
}
