use crate::prelude::*;

pub(super) struct GameStatusPlugin;

impl Plugin for GameStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (health_delete, armor_delete, health_decr));
    }
}

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub hp: u32,
    pub remain: u32,
    stack: Vec<u32>,
}
impl Health {
    pub fn new(hp: u32, remain: u32) -> Self {
        Self {
            hp,
            remain,
            stack: Vec::new(),
        }
    }

    pub fn true_decr(&mut self, value: u32) {
        if self.hp >= value {
            self.hp -= value;
        } else {
            self.hp = 0;
            self.remain = self.remain.wrapping_sub(value);
        }
    }

    pub fn true_incr(&mut self, value: u32) {
        self.hp += value;
    }

    pub fn is0(&self) -> bool {
        self.hp == 0 && self.remain == 0
    }

    // This decreases the armors first(if any)
    pub fn decr(&mut self, value: u32) {
        self.stack.push(value);
    }
}

#[derive(Component, Debug, Clone)]
pub struct Armor {
    pub hp: u32,
}
impl Armor {
    pub fn new(hp: u32) -> Self {
        Self { hp }
    }

    pub fn decr(&mut self, value: u32) {
        self.hp = self.hp.wrapping_sub(value);
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

fn armor_delete(mut commands: Commands, q_armor: Query<(Entity, &Armor)>) {
    q_armor.iter().for_each(|(entity, armor)| {
        if armor.hp == 0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}

fn health_decr(mut q_health: Query<(&mut Health, &Children)>, q_armor: Query<&mut Armor>) {
    let q_armor = RwLock::new(q_armor);
    q_health.par_iter_mut().for_each(|(mut health, children)| {
        if !health.stack.is_empty() {
            let mut sum = 0;
            for hp in health.stack.drain(..) {
                let mut ok = false;
                for entity in children.iter() {
                    if let Ok(mut armor) = q_armor.write().unwrap().get_mut(*entity) {
                        armor.decr(hp);
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    sum += hp;
                }
            }
            health.true_decr(sum);
        }
    });
}
