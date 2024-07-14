use crate::prelude::*;

pub(super) struct PlantsPeaPlugin;

impl Plugin for PlantsPeaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (init_config,));
    }
}

game_conf!(projectile ProjectilePea);
game_conf!(creature Peashooter);

fn init_config(
    mut commands: Commands,
    plants: Res<assets::SpritePlants>,
    mut map: ResMut<game::CreatureMap>,
) {
    commands.insert_resource(ProjectilePea(Arc::new(game::ProjectileShared {
        anim: plants.pea.clone(),
        hitbox: game::HitBox::new(50.0, 50.0),
    })));
    // TODO: Peashooter config
}
