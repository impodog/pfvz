use crate::prelude::*;

pub(super) struct PlantsMushroomPlugin;

impl Plugin for PlantsMushroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (modify_mushroom, modify_zzz).run_if(when_state!(gaming)),
        );
    }
}

/// Mushroom(is_sleeping), defaults to true
#[derive(Component, Deref, DerefMut)]
pub struct Mushroom(pub bool);
impl Default for Mushroom {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub struct MushroomZzzMarker;

fn modify_mushroom(
    mut q_mushroom: Query<&mut Mushroom, Added<Mushroom>>,
    level: Res<level::Level>,
) {
    q_mushroom.iter_mut().for_each(|mut mushroom| {
        mushroom.0 = !level.config.layout.is_night();
    });
}

fn modify_zzz(
    mut commands: Commands,
    mut q_mushroom: Query<
        (
            Entity,
            Option<&Children>,
            &Mushroom,
            &game::HitBox,
            &mut game::Overlay,
        ),
        Changed<Mushroom>,
    >,
    q_zzz: Query<Entity, With<MushroomZzzMarker>>,
    plants: Res<assets::SpritePlants>,
) {
    q_mushroom
        .iter_mut()
        .for_each(|(entity, children, mushroom, hitbox, mut overlay)| {
            if mushroom.0 {
                commands
                    .spawn((
                        MushroomZzzMarker,
                        // This overlay prevents animation freezing from the parent overlay
                        game::Overlay::default(),
                        game::Position::default(),
                        *hitbox,
                        sprite::Animation::new(plants.zzz.clone()),
                        game::LayerDisp(0.02),
                        SpriteBundle::default(),
                    ))
                    .set_parent(entity);
                overlay.multiply(0.0);
            } else {
                #[allow(clippy::collapsible_else_if)]
                if let Some(children) = children {
                    if let Some(zzz) = children.iter().find(|entity| q_zzz.get(**entity).is_ok()) {
                        commands.entity(entity).remove_children(&[*zzz]);
                        commands.entity(*zzz).despawn_recursive();
                    }
                }
                overlay.divide(0.0);
            }
        });
}
