use crate::prelude::*;

pub(super) struct GameStatusPlugin;

impl Plugin for GameStatusPlugin {
    fn build(&self, app: &mut App) {}
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
}
