use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::GlobalStates::Play),
            (init_player_status, show_selection),
        );
        app.init_resource::<Sun>();
        app.init_resource::<Selection>();
        app.init_resource::<Selecting>();
    }
}

#[derive(Resource, Default, Debug, Clone, Deref, DerefMut)]
pub struct Sun(pub u32);

#[derive(Resource, Serialize, Deserialize, Default, Debug, Clone, Deref, DerefMut)]
pub struct Selection(pub Vec<Id>);

#[derive(Resource, Default, Debug, Clone, Copy, Deref, DerefMut)]
pub struct Selecting(pub Id);

#[derive(Component)]
struct SelectionMarker;

fn init_player_status(mut commands: Commands) {
    commands.insert_resource(Sun::default());
    commands.insert_resource(Selection::default());
    commands.insert_resource(Selecting::default());
}

fn show_selection(
    mut commands: Commands,
    sel: Res<Selection>,
    map: Res<game::CreatureMap>,
    display: Res<game::Display>,
    q_sel: Query<Entity, With<SelectionMarker>>,
) {
    if sel.is_changed() {
        q_sel.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
        for (i, id) in sel.iter().enumerate() {
            if let Some(creature) = map.get(id) {
                commands.spawn((
                    SelectionMarker,
                    SpriteBundle {
                        texture: creature
                            .anim
                            .frames
                            .first()
                            .expect("Empty animation!")
                            .clone(),
                        sprite: Sprite {
                            custom_size: Some(SLOT_SIZE * display.ratio),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    sprite::SlotIndex(i).into_position(display.ratio),
                ));
            } else {
                warn!("Selected non-existing id: {}", id);
            }
        }
    }
}
