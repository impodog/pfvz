use crate::prelude::*;

pub(super) struct GameCreaturePlugin;

impl Plugin for GameCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatureAction>();
        app.init_resource::<CreatureMap>();
        app.add_systems(
            Update,
            (creature_action, creature_update).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Creature(Arc<CreatureShared>);

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct CreatureMap(pub HashMap<Id, Creature>);

#[derive(Debug, Clone)]
pub struct CreatureShared {
    pub cost: u32,
    pub anim: Arc<sprite::FrameArr>,
    pub spawn: SystemId<game::Position>,
    pub die: SystemId<Entity>,
    pub update: SystemId<Entity>,
    pub damage: SystemId<(Entity, u32)>,
}

#[derive(Event, Debug, Clone)]
pub enum CreatureAction {
    Spawn(Creature, game::Position),
    Die(Entity),
    Damage(Entity, u32),
}

fn creature_action(
    mut commands: Commands,
    mut e_action: EventReader<CreatureAction>,
    q_creature: Query<&Creature>,
) {
    e_action.read().for_each(|action| {
        let ok = match action {
            CreatureAction::Spawn(creature, pos) => {
                commands.run_system_with_input(creature.spawn, *pos);
                true
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

fn creature_update(mut commands: Commands, q_creature: Query<(Entity, &Creature)>) {
    q_creature.iter().for_each(|(entity, creature)| {
        commands.run_system_with_input(creature.update, entity);
    });
}
