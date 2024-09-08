use crate::prelude::*;

pub(super) struct InfoMousePlugin;

impl Plugin for InfoMousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>();
        app.add_systems(
            PreUpdate,
            (init_cursor_info, update_cursor_info, update_touch_as_cursor).chain(),
        );
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct CursorInfo {
    coord: Vec2,
    pub pos: game::Position,
    pub left: bool,
    pub right: bool,
}

fn init_cursor_info(mut cursor: ResMut<CursorInfo>) {
    cursor.left = false;
    cursor.right = false;
}

fn update_cursor_info(
    mut cursor: ResMut<CursorInfo>,
    display: Res<game::Display>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<config::MainCamera>>,
    button: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor.coord = world_position;
        cursor.pos = game::Position::new_xy(
            cursor.coord.x / display.ratio,
            cursor.coord.y / display.ratio,
        );
    }
    cursor.left |= button.just_pressed(MouseButton::Left);
    cursor.right |= button.just_pressed(MouseButton::Right);
}

fn update_touch_as_cursor(
    mut cursor: ResMut<CursorInfo>,
    display: Res<game::Display>,
    q_camera: Query<(&Camera, &GlobalTransform), With<config::MainCamera>>,
    touches: Res<Touches>,
) {
    let (camera, camera_transform) = q_camera.single();

    touches.iter_just_released().for_each(|touch| {
        let pos = touch.position();
        let start = touch.start_position();
        if pos.distance_squared(start) >= LOGICAL_BOUND.length_squared() / 4.0 {
            cursor.right = true;
        } else {
            cursor.left = true;
        }

        if let Some(world_position) = camera
            .viewport_to_world(camera_transform, pos)
            .map(|ray| ray.origin.truncate())
        {
            cursor.coord = world_position;
            cursor.pos = game::Position::new_xy(
                cursor.coord.x / display.ratio,
                cursor.coord.y / display.ratio,
            );
        }
    });
}
