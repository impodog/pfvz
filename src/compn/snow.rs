use crate::prelude::*;

pub(super) struct CompnSnowPlugin;

impl Plugin for CompnSnowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_snow, snowy_bump, remove_snow).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone)]
pub struct Snow {
    pub duration: Duration,
    // The speed factor to multiply, e.g. 0.5
    pub factor: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct SnowSerde {
    pub duration: f32,
    pub factor: f32,
}
impl From<SnowSerde> for Snow {
    fn from(values: SnowSerde) -> Self {
        Self {
            duration: Duration::from_secs_f32(values.duration),
            factor: values.factor,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct SnowyProjectile {
    pub snow: Snow,
}

#[derive(Component, Debug, Clone)]
pub struct SnowImpl {
    pub timer: Timer,
}

fn snowy_bump(
    mut commands: Commands,
    mut action: EventReader<game::ProjectileAction>,
    q_snow: Query<&SnowyProjectile>,
) {
    action.read().for_each(|action| {
        if let game::ProjectileAction::Damage(entity, other) = action {
            if let Ok(snowy) = q_snow.get(*entity) {
                if let Some(mut commands) = commands.get_entity(*other) {
                    commands.try_insert(snowy.snow.clone());
                }
            }
        }
    });
}

fn add_snow(
    mut commands: Commands,
    mut q_snow: Query<(Entity, &Snow, &mut game::Overlay), Changed<Snow>>,
    mut q_snow_impl: Query<&mut SnowImpl>,
) {
    q_snow.iter_mut().for_each(|(entity, snow, mut overlay)| {
        if let Ok(mut snow_impl) = q_snow_impl.get_mut(entity) {
            if snow_impl.timer.remaining() < snow.duration {
                snow_impl.timer.set_duration(snow.duration);
                snow_impl.timer.reset();
            }
        } else {
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.insert(SnowImpl {
                    timer: Timer::new(snow.duration, TimerMode::Once),
                });
            }
            overlay.multiply(snow.factor);
        }
    });
}

fn remove_snow(
    mut commands: Commands,
    mut q_snow: Query<(Entity, &mut game::Overlay, &Snow, &mut SnowImpl)>,
    actual_time: Res<config::FrameTime>,
) {
    q_snow
        .iter_mut()
        .for_each(|(entity, mut overlay, snow, mut snow_imp)| {
            // We use actual time here to detach the snow timer from the snow effect
            snow_imp.timer.tick(actual_time.delta());
            if snow_imp.timer.just_finished() {
                overlay.divide(snow.factor);
                commands
                    .entity(entity)
                    .remove::<Snow>()
                    .remove::<SnowImpl>();
            }
        })
}
