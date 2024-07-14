use crate::prelude::*;

#[macro_export]
macro_rules! game_conf {
    (projectile $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::game::ProjectileShared>);
    };
    (shooter $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::compn::ShooterShared>);
    };
    (systems $name: ident) => {
        lazy_static! {
            static ref $name: RwLock<Option<$crate::game::CreatureSystems>> = RwLock::new(None);
        }
    };
}
