use crate::prelude::*;

pub(super) struct GameCreaturePlugin;

impl Plugin for GameCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatureAction>();
        app.init_resource::<CreatureMap>();
        app.add_systems(Update, (creature_action,).run_if(when_state!(gaming)));
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Creature(pub Arc<CreatureShared>);

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct CreatureMap(pub HashMap<Id, Creature>);

#[derive(Debug, Clone, Copy)]
pub struct CreatureSystems {
    pub spawn: SystemId<game::LogicPosition>,
    pub die: SystemId<Entity>,
    pub damage: SystemId<(Entity, u32)>,
}
impl Default for CreatureSystems {
    fn default() -> Self {
        Self {
            spawn: compn::default::system_spawn_not.read().unwrap().unwrap(),
            die: compn::default::system_die.read().unwrap().unwrap(),
            damage: compn::default::system_damage.read().unwrap().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct CreatureShared {
    #[deref]
    pub systems: CreatureSystems,
    pub cost: u32,
    pub cooldown: f32,
    pub image: Handle<Image>,
    pub hitbox: game::HitBox,
    pub flags: level::CreatureFlags,
}

#[derive(Event, Debug, Clone)]
pub enum CreatureAction {
    Spawn(Id, game::LogicPosition),
    Die(Entity),
    Damage(Entity, u32),
}

fn creature_action(
    mut commands: Commands,
    map: Res<CreatureMap>,
    mut e_action: EventReader<CreatureAction>,
    q_creature: Query<&Creature>,
) {
    e_action.read().for_each(|action| {
        let ok = match action {
            CreatureAction::Spawn(id, pos) => {
                if let Some(creature) = map.get(id) {
                    commands.run_system_with_input(creature.spawn, *pos);
                    true
                } else {
                    false
                }
            }
            CreatureAction::Die(entity) => {
                if let Ok(creature) = q_creature.get(*entity) {
                    commands.run_system_with_input(creature.die, *entity);
                    true
                } else {
                    false
                }
            }
            CreatureAction::Damage(entity, damage) => {
                if let Ok(creature) = q_creature.get(*entity) {
                    commands.run_system_with_input(creature.damage, (*entity, *damage));
                    true
                } else {
                    false
                }
            }
        };
        if !ok {
            warn!("Unable to execute creature action: {:?}", action);
        }
    });
}
