use crate::prelude::*;

pub(super) struct InfoMousePlugin;

impl Plugin for InfoMousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorInfo>();
        app.add_systems(
            PreUpdate,
            ((
                update_cursor_info,
                update_inbound.run_if(in_state(info::GlobalStates::Play)),
            ),),
        );
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct CursorInfo {
    coord: Vec2,
    pub pos: game::Position,
    pub left: bool,
    pub right: bool,
    pub inbound: bool,
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
    cursor.left = button.just_pressed(MouseButton::Left);
    cursor.right = button.just_pressed(MouseButton::Right);
}

fn update_inbound(mut cursor: ResMut<CursorInfo>, level: Res<level::Level>) {
    let size = level.config.layout.size_f32();
    cursor.inbound = cursor.pos.x >= -size.0 / 2.0
        && cursor.pos.x <= size.0 / 2.0
        && cursor.pos.y >= -size.1 / 2.0
        && cursor.pos.y <= size.1 / 2.0;
}
