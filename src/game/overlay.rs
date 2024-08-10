use crate::prelude::*;

pub(super) struct GameOverlayPlugin;

impl Plugin for GameOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (add_overlay,));
        app.add_systems(PreUpdate, (update_overlay,));
    }
}

#[derive(Component, Default, Debug, Clone, Deref, DerefMut)]
pub struct Overlay {
    #[deref]
    speed: game::Factor,
    delta: Duration,
}

impl Overlay {
    pub fn speed(&self) -> f32 {
        self.speed.factor()
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }
}

fn add_overlay(mut commands: Commands, q_creature: Query<Entity, Added<game::Creature>>) {
    q_creature.iter().for_each(|entity| {
        commands.entity(entity).insert(Overlay::default());
    });
}

fn update_overlay(time: Res<config::FrameTime>, mut q_overlay: Query<&mut Overlay>) {
    q_overlay.par_iter_mut().for_each(|mut overlay| {
        overlay.delta = Duration::from_secs_f32(time.delta().as_secs_f32() * overlay.speed());
    });
}
