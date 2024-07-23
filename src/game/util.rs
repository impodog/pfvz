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
    (producer $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::compn::ProducerShared>);
    };
    (walker $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::compn::WalkerShared>);
    };
    (breaks $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::compn::BreaksShared>);
    };
    (explode $name: ident) => {
        #[derive(Resource, Debug, Clone, Deref, DerefMut)]
        pub struct $name(Arc<$crate::compn::ExplodeShared>);
    };
    (systems $name: ident) => {
        lazy_static! {
            static ref $name: RwLock<Option<$crate::game::CreatureSystems>> = RwLock::new(None);
        }
    };
}

#[macro_export]
macro_rules! debug_spawn_system {
    ($id: ident, $x: literal, $y:literal) => {
        |mut action: EventWriter<game::CreatureAction>, mut b: Local<bool>| {
            if !*b {
                *b = true;
                action.send(game::CreatureAction::Spawn(
                    $id,
                    game::Position::new_xy($x, $y),
                ));
            }
        }
    };
}
