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
    (system $name: ident, $in: ty) => {
        lazy_static! {
            static ref $name: RwLock<Option<bevy::ecs::system::SystemId<$in>>> = RwLock::new(None);
        }
    };
    (pub system $name: ident, $in: ty) => {
        lazy_static! {
            pub static ref $name: RwLock<Option<bevy::ecs::system::SystemId<$in>>> =
                RwLock::new(None);
        }
    };
}

pub fn query_overlay<F, R>(
    f: F,
    entity: Entity,
    q_overlay: &Query<&game::Overlay>,
    q_parent: &Query<&Parent>,
) -> R
where
    F: FnOnce(Option<&game::Overlay>) -> R,
    R: 'static,
{
    if let Ok(overlay) = q_overlay.get(entity) {
        f(Some(overlay))
    } else if let Ok(parent) = q_parent.get(entity) {
        query_overlay(f, parent.get(), q_overlay, q_parent)
    } else {
        f(None)
    }
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
