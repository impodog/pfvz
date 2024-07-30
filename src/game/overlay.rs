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

    // Whenever a division by 0.0 occurs, use this to get the accurate result of speed
    pub fn recalculate(&mut self) {
        self.speed = 1.0;
        for (value, times) in self.speed_queue.iter() {
            self.speed *= value.0 * (*times as f32);
        }
    }

    pub fn multiply(&mut self, rate: f32) {
        match self.speed_queue.entry(rate.into()) {
            std::collections::btree_map::Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
            std::collections::btree_map::Entry::Occupied(mut occupied) => {
                *occupied.get_mut() += 1;
            }
        }
        self.speed *= rate;
    }

    pub fn divide(&mut self, rate: f32) {
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
            // This prevents dividing by 0
            if rate == 0.0 {
                self.recalculate();
            } else {
                self.speed /= rate;
            }
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
