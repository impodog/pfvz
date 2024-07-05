use crate::prelude::*;

pub(super) struct InfoMousePlugin;

impl Plugin for InfoMousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>();
        app.add_systems(PreUpdate, (update_cursor_info,));
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct CursorInfo {
    pos: Vec2,
}

fn update_cursor_info(
    mut cursor: ResMut<CursorInfo>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<config::MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor.pos = world_position;
    }
}
