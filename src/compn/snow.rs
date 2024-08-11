use crate::prelude::*;

pub(super) struct CompnSnowPlugin;

impl Plugin for CompnSnowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (snowy_bump, modify_snow, snow_timer, remove_snow_from_parent)
                .run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone)]
pub struct Snow {
    pub duration: Duration,
    // The speed factor to multiply, e.g. 0.5
    pub factor: f32,
}
#[derive(Component, Debug, Clone)]
pub enum ModifySnow {
    Add(Snow),
    Remove,
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
impl From<&Snow> for SnowImpl {
    fn from(snow: &Snow) -> Self {
        Self {
            timer: Timer::new(snow.duration, TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct UnsnowParent {
    // If absolute, no snow effect is applied, otherwise only >0.0 effects are applied
    pub absolute: bool,
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
                    commands.try_insert(ModifySnow::Add(snowy.snow.clone()));
                }
            }
        }
    });
}

fn modify_snow(
    commands: ParallelCommands,
    mut q_modify: Query<(Entity, &mut game::Overlay, &ModifySnow)>,
    q_snow: Query<(&Snow, &SnowImpl)>,
) {
    q_modify
        .par_iter_mut()
        .for_each(|(entity, mut overlay, modify)| {
            match modify {
                ModifySnow::Add(snow) => {
                    let ok = if let Ok((prev_snow, prev_snow_impl)) = q_snow.get(entity) {
                        if prev_snow.factor > snow.factor
                            || (prev_snow.factor == snow.factor
                                && prev_snow_impl.timer.remaining() < snow.duration)
                        {
                            overlay.divide(prev_snow.factor);
                            overlay.multiply(snow.factor);

                            true
                        } else {
                            false
                        }
                    } else {
                        overlay.multiply(snow.factor);
                        true
                    };
                    if ok {
                        commands.command_scope(|mut commands| {
                            commands
                                .entity(entity)
                                .try_insert(snow.clone())
                                .try_insert(SnowImpl::from(snow));
                        });
                    }
                }
                ModifySnow::Remove => {
                    commands.command_scope(|mut commands| {
                        if let Ok((prev_snow, _prev_snow_impl)) = q_snow.get(entity) {
                            overlay.divide(prev_snow.factor);
                            commands
                                .entity(entity)
                                .remove::<Snow>()
                                .remove::<SnowImpl>();
                        }
                    });
                }
            }
            commands.command_scope(|mut commands| {
                commands.entity(entity).remove::<ModifySnow>();
            })
        });
}

fn snow_timer(
    commands: ParallelCommands,
    time: Res<config::FrameTime>,
    mut q_snow: Query<(Entity, &mut SnowImpl)>,
) {
    q_snow.par_iter_mut().for_each(|(entity, mut snow_impl)| {
        snow_impl.timer.tick(time.delta());
        if snow_impl.timer.just_finished() {
            commands.command_scope(|mut commands| {
                commands.entity(entity).try_insert(ModifySnow::Remove);
            });
        }
    });
}

fn remove_snow_from_parent(
    mut commands: Commands,
    q_parent: Query<(&UnsnowParent, &Parent)>,
    q_snow: Query<&Snow, With<SnowImpl>>,
) {
    q_parent.iter().for_each(|(unsnow, parent)| {
        if let Ok(snow) = q_snow.get(parent.get()) {
            if unsnow.absolute || snow.factor != 0.0 {
                commands.entity(parent.get()).try_insert(ModifySnow::Remove);
            }
        }
    });
}
