use crate::prelude::*;

#[macro_export]
macro_rules! game_conf {
    (projectile $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<game::ProjectileShared>);
    };
    (shooter $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<compn::ShooterShared>);
    };
    (creature $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<game::Creature>);
    };
}
