use crate::prelude::*;

pub(super) struct ConfigCameraPlugin;

impl Plugin for ConfigCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera,));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
