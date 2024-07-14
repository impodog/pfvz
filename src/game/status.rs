use crate::prelude::*;

pub(super) struct GameStatusPlugin;

impl Plugin for GameStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (health_delete,));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub hp: u32,
    pub remain: u32,
}
impl Health {
    pub fn decr(&mut self, value: u32) {
        if self.hp >= value {
            self.hp -= value;
        } else {
            self.hp = 0;
            self.remain = self.remain.wrapping_sub(value);
        }
    }

    pub fn incr(&mut self, value: u32) {
        self.hp += value;
    }

    pub fn is0(&self) -> bool {
        self.hp == 0 && self.remain == 0
    }
}

fn health_delete(
    mut e_action: EventWriter<game::CreatureAction>,
    q_health: Query<(Entity, &Health), With<game::Creature>>,
) {
    q_health.iter().for_each(|(entity, health)| {
        if health.is0() {
            e_action.send(game::CreatureAction::Die(entity));
        }
    });
}
