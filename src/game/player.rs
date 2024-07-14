use crate::prelude::*;

pub(super) struct GamePlayerPlugin;

impl Plugin for GamePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sun>();
        app.init_resource::<Selection>();
        app.add_systems(OnEnter(info::GlobalStates::Play), (init_player_status,));
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Sun(u32);

#[derive(Resource, Default, Debug, Clone)]
pub struct Selection(Vec<Id>);

fn init_player_status(mut commands: Commands) {
    commands.insert_resource(Sun::default());
    commands.insert_resource(Selection::default());
}
