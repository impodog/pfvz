use crate::prelude::*;

pub(super) struct LevelDispPlugin;

impl Plugin for LevelDispPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (add_height_displace, update_displace).run_if(when_state!(gaming)),
        );
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct HeightDisplace(pub f32);

#[derive(Component)]
pub struct ForceDisplace;

pub type CanDisplace = Or<(With<game::Creature>, With<ForceDisplace>)>;

fn add_height_displace(
    mut commands: Commands,
    q_displace: Query<(Entity, &game::LogicPosition), (Added<game::LogicPosition>, CanDisplace)>,
    level: Res<level::Level>,
) {
    q_displace.iter().for_each(|(entity, logic)| {
        let (x, _y) = level
            .config
            .layout
            .position_3d_to_coordinates(logic.base_raw());
        if let Some(mut commands) = commands.get_entity(entity) {
            commands.try_insert(HeightDisplace(level.config.layout.get_disp(x)));
        }
    })
}

fn update_displace(
    mut q_displace: Query<(&mut game::LogicPosition, &mut HeightDisplace)>,
    level: Res<level::Level>,
) {
    q_displace
        .par_iter_mut()
        .for_each(|(mut logic, mut displace)| {
            let (x, _y) = level
                .config
                .layout
                .position_3d_to_coordinates(logic.base_raw());
            let disp = level.config.layout.get_disp(x);
            let diff = disp - displace.0;
            if diff.abs() > f32::EPSILON {
                logic.base_raw_mut().z += diff;
                displace.0 = disp;
            }
        });
}
