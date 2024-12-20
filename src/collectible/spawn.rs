use crate::prelude::*;

pub(super) struct CollectibleSpawnPlugin;

impl Plugin for CollectibleSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (init_timer,));
        app.add_systems(Update, (spawn_sun,).run_if(when_state!(gaming)));
    }
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
struct SunTimer(Timer);

fn init_timer(mut commands: Commands, factors: Res<collectible::ItemFactors>) {
    commands.insert_resource(SunTimer(Timer::new(
        Duration::from_secs_f32(factors.sun.interval),
        TimerMode::Repeating,
    )));
}

fn spawn_sun(
    mut commands: Commands,
    factors: Res<collectible::ItemFactors>,
    time: Res<config::FrameTime>,
    mut timer: ResMut<SunTimer>,
    level: Res<level::Level>,
) {
    timer.tick(time.delta());
    if level.config.is_sun_spawn() && timer.just_finished() {
        let size = level.config.layout.size_f32();
        let pos = game::Position::new(
            rand::thread_rng().gen_range(-size.0 / 2.0..size.0 / 2.0),
            rand::thread_rng().gen_range(-size.1 / 2.0..size.1 / 2.0),
            factors.sun.height,
            0.0,
        );
        commands.spawn((
            pos,
            collectible::Collectible::Sun(1.0),
            factors.sun.velocity,
        ));
    }
}
