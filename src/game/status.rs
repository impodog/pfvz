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
            self.remain = self.remain.saturating_sub(value - self.hp);
        }
    }

    pub fn true_incr(&mut self, value: u32) {
        self.hp += value;
    }

    pub fn true_decr_hp_only(&mut self, value: u32) {
        self.hp = self.hp.saturating_sub(value)
    }

    pub fn is_zero(&self) -> bool {
        self.hp == 0 && self.remain == 0
    }

    pub fn is_dying(&self) -> bool {
        self.hp == 0
    }

    // This decreases the armors first(if any)
    pub fn decr(&mut self, value: u32) {
        self.stack.push(value);
    }

    pub fn value(&self) -> u32 {
        self.hp + self.remain
    }
}
impl From<u32> for Health {
    fn from(value: u32) -> Self {
        Self::new(value, 0)
    }
}
impl From<(u32, u32)> for Health {
    fn from(value: (u32, u32)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}", self.hp, self.remain)
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

    pub fn decr(&mut self, value: u32) -> u32 {
        let overflow = value.saturating_sub(self.hp);
        self.hp = self.hp.saturating_sub(value);
        overflow
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct HealthDeleteTimer(pub Timer);

impl Default for HealthDeleteTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
}

fn health_delete(
    commands: ParallelCommands,
    action: EventWriter<game::CreatureAction>,
    mut q_health: Query<(Entity, &Health, Option<&mut HealthDeleteTimer>), With<game::Creature>>,
    time: Res<config::FrameTime>,
) {
    let action = Mutex::new(action);
    q_health.par_iter_mut().for_each(|(entity, health, timer)| {
        if health.is_zero() {
            let ok = if let Some(mut timer) = timer {
                timer.tick(time.delta());
                timer.just_finished()
            } else {
                commands.command_scope(|mut commands| {
                    if let Some(mut commands) = commands.get_entity(entity) {
                        commands.try_insert(HealthDeleteTimer::default());
                    }
                });
                true
            };
            if ok {
                action
                    .lock()
                    .unwrap()
                    .send(game::CreatureAction::Die(entity));
            }
        }
    });
}

fn armor_delete(commands: ParallelCommands, q_armor: Query<(Entity, &Armor)>) {
    q_armor.par_iter().for_each(|(entity, armor)| {
        if armor.hp == 0 {
            commands.command_scope(|mut commands| {
                if let Some(commands) = commands.get_entity(entity) {
                    commands.despawn_recursive();
                }
            });
        }
    });
}

fn health_decr(
    mut q_health: Query<(Entity, &mut Health)>,
    q_children: Query<&Children>,
    q_armor: Query<&mut Armor>,
    q_dying: Query<(), With<compn::Dying>>,
) {
    let q_armor = Mutex::new(q_armor);
    q_health.par_iter_mut().for_each(|(entity, mut health)| {
        if !health.stack.is_empty() {
            let mut sum = 0;
            for hp in health.stack.drain(..) {
                let mut ok = false;
                if let Ok(children) = q_children.get(entity) {
                    for entity in children.iter() {
                        if let Ok(mut armor) = q_armor.lock().unwrap().get_mut(*entity) {
                            sum += armor.decr(hp);
                            ok = true;
                            break;
                        }
                    }
                }
                if !ok {
                    sum += hp;
                }
            }
            if q_dying.get(entity).is_ok() {
                health.true_decr_hp_only(sum);
            } else {
                health.true_decr(sum);
            }
        }
    });
}
