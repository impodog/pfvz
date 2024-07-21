use crate::prelude::*;

pub(super) struct CollectibleCollPlugin;

impl Plugin for CollectibleCollPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectibleEvent>();
        app.add_systems(Update, (test_tap,));
    }
}

#[derive(Component, Debug, Clone)]
pub enum Collectible {
    /// When collected, get sun value equal to sun_value(from config) * self.0
    Sun(f32),
}

#[derive(Event, Debug, Clone)]
pub struct CollectibleEvent {
    pub entity: Entity,
}

fn test_tap(
    mut e_coll: EventWriter<CollectibleEvent>,
    q_coll: Query<(Entity, &game::Position, &game::HitBox), With<Collectible>>,
    cursor: Res<info::CursorInfo>,
) {
    if cursor.left {
        q_coll.iter().for_each(|(entity, pos, hitbox)| {
            if (cursor.pos.x - pos.x).abs() <= hitbox.width
                && (cursor.pos.y - pos.y - pos.z).abs() <= hitbox.height
            {
                e_coll.send(CollectibleEvent { entity });
            }
        });
    }
}
