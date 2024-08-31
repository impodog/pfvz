use crate::prelude::*;

pub(super) struct ModesColumnsPlugin;

impl Plugin for ModesColumnsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (plant_rows,).run_if(when_state!(gaming)));
    }
}

fn plant_rows(
    mut e_plant: EventReader<plants::PlanterEvent>,
    mut call: EventWriter<plants::PlanterCall>,
    level: Res<level::Level>,
) {
    if level.config.game.contains(&level::GameKind::Columns) {
        e_plant.read().for_each(|plant| {
            let size = level.config.layout.size();
            let (x, y) = level.config.layout.position_3d_to_coordinates(&plant.base);
            for row in (0..size.1).filter(|row| *row != y) {
                call.send(plants::PlanterCall {
                    id: plant.id,
                    coordinates: (x, row),
                    ..Default::default()
                });
            }
        });
    }
}
