use crate::prelude::*;

pub(super) struct CollectibleCollectPlugin;

impl Plugin for CollectibleCollectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (read_events,));
    }
}

fn read_events(
    mut commands: Commands,
    mut e_coll: EventReader<collectible::CollectibleEvent>,
    config: Res<config::Config>,
    mut sun: ResMut<game::Sun>,
    q_coll: Query<&collectible::Collectible>,
    audio: Res<Audio>,
    audio_items: Res<assets::AudioItems>,
) {
    e_coll.read().for_each(|event| {
        if let Ok(coll) = q_coll.get(event.entity) {
            match coll {
                collectible::Collectible::Nothing => {}
                collectible::Collectible::Sun(value) => {
                    sun.0 += (config.gamerule.sun_value.0 as f32 * *value) as u32;
                    audio.play(audio_items.sun.random());
                }
            }
            commands.entity(event.entity).despawn_recursive();
        } else {
            warn!("Unable to execute collectible event: {:?}", event);
        }
    });
}
