use crate::prelude::*;

pub(super) struct AlmanacShowPlugin;

impl Plugin for AlmanacShowPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AlmanacShowPicture>();
        app.add_systems(OnEnter(info::MenuStates::Almanac), (init_display,));
        app.add_systems(PostUpdate, (show_picture,));
    }
}

fn init_display(mut commands: Commands) {
    initialize(&level::FIXED_RATIO);
    commands.insert_resource(game::Display {
        ratio: *level::FIXED_RATIO,
    });
}

#[derive(Event)]
pub struct AlmanacShowPicture {
    pub id: Id,
    pub pos: game::Position,
    pub hitbox: game::HitBox,
    pub vis: Visibility,
    pub item: AlmanacItem,
}

#[derive(Component, Clone, Copy)]
pub struct AlmanacItem {
    pub page: usize,
}

fn show_picture(
    mut e_show: EventReader<AlmanacShowPicture>,
    mut commands: Commands,
    map: Res<game::CreatureMap>,
) {
    e_show.read().for_each(|event| {
        if let Some(creature) = map.get(&event.id) {
            commands.spawn((
                event.item,
                event.pos,
                event.hitbox,
                SpriteBundle {
                    texture: creature.image.clone(),
                    sprite: Sprite {
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    visibility: event.vis,
                    ..Default::default()
                },
            ));
        } else {
            error!("Attempt to show unknown creature id {}", event.id);
        }
    });
}
