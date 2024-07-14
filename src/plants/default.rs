use crate::prelude::*;

pub(super) fn default_spawn(mut commands: Commands, In(pos): In<game::Position>) {
    commands.spawn((pos,));
}
