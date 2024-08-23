use crate::prelude::*;

pub(super) struct ModesRoofPlugin;

impl Plugin for ModesRoofPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::PlayStates::Gaming), (add_flower_pots,));
    }
}

fn add_flower_pots(mut planter: EventWriter<plants::PlanterCall>, level: Res<level::Level>) {
    if matches!(level.config.layout, level::LayoutKind::Roof) {
        let cols = level.config.layout.size().1;
        for row in 0..3 {
            for col in 0..cols {
                planter.send(plants::PlanterCall {
                    coordinates: (row, col),
                    id: FLOWER_POT,
                    ..Default::default()
                });
            }
        }
    }
}
