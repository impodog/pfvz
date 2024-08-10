use crate::prelude::*;

pub(super) struct CompnWaterPlugin;

impl Plugin for CompnWaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_in_water, put_in_water));
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct InWater(pub bool);

fn add_in_water(
    commands: ParallelCommands,
    q_zombie: Query<(Entity, &game::Position), Added<game::Zombie>>,
    level: Res<level::Level>,
) {
    q_zombie.par_iter().for_each(|(entity, pos)| {
        if level
            .config
            .layout
            .get_lane(level.config.layout.position_to_coordinates(pos).1)
            == level::TileFeature::Water
        {
            commands.command_scope(|mut commands| {
                commands.entity(entity).try_insert(InWater::default());
            });
        }
    });
}

fn put_in_water(
    mut q_zombie: Query<(&game::Position, &mut game::Size, &mut InWater), With<game::Zombie>>,
    level: Res<level::Level>,
) {
    q_zombie
        .par_iter_mut()
        .for_each(|(pos, mut size, mut in_water)| {
            let (x, y) = level.config.layout.position_to_coordinates(pos);
            let status = level.config.layout.get_tile(x, y) == level::TileFeature::Water;
            if status != **in_water {
                **in_water = status;
                if status {
                    size.y_mut().multiply(0.6);
                } else {
                    size.y_mut().divide(0.6);
                }
            }
        });
}
