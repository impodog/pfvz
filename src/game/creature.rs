use crate::prelude::*;

pub(super) struct GameCreaturePlugin;

impl Plugin for GameCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatureAction>();
        app.init_resource::<CreatureTypes>();
        app.add_systems(
            Update,
            (creature_action, creature_update).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Creature(pub Id);

#[derive(Debug, Clone)]
pub struct CreatureFun {
    pub spawn: SystemId<game::Position>,
    pub die: SystemId,
    pub update: SystemId<Entity>,
    pub damage: SystemId<(Entity, u32)>,
    pub cost: u32,
}

#[derive(Resource, Default, Debug)]
pub struct CreatureTypes(pub HashMap<Id, CreatureFun>);

#[derive(Event, Debug, Clone)]
pub enum CreatureAction {
    Spawn(Id, game::Position),
    Die(Entity),
    Damage(Entity, u32),
}

fn creature_action(
    mut commands: Commands,
    types: Res<CreatureTypes>,
    mut e_action: EventReader<CreatureAction>,
    q_creature: Query<&Creature>,
) {
    e_action.read().for_each(|action| {
        let ok = match action {
            CreatureAction::Spawn(id, pos) => {
                if let Some(fun) = types.0.get(id) {
                    commands.run_system_with_input(fun.spawn, *pos);
                    true
                } else {
                    false
                }
            }
            CreatureAction::Die(entity) => {
                if let Some(fun) = q_creature
                    .get(*entity)
                    .ok()
                    .and_then(|creature| types.0.get(&creature.0))
                {
                    commands.run_system(fun.die);
                    true
                } else {
                    false
                }
            }
            CreatureAction::Damage(entity, damage) => {
                if let Some(fun) = q_creature
                    .get(*entity)
                    .ok()
                    .and_then(|creature| types.0.get(&creature.0))
                {
                    commands.run_system_with_input(fun.damage, (*entity, *damage));
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

fn creature_update(
    mut commands: Commands,
    types: Res<CreatureTypes>,
    q_creature: Query<(Entity, &Creature)>,
) {
    let commands_vec = Arc::new(RwLock::new(Vec::new()));
    q_creature.par_iter().for_each(|(entity, creature)| {
        if let Some(fun) = types.0.get(&creature.0) {
            commands_vec.write().unwrap().push(
                bevy::ecs::system::RunSystemWithInput::new_with_input(fun.update, entity),
            );
        }
    });
    let commands_vec = Arc::into_inner(commands_vec).unwrap();
    let commands_vec = RwLock::into_inner(commands_vec).unwrap();
    for command in commands_vec.into_iter() {
        commands.add(command);
    }
}
