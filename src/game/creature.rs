use crate::prelude::*;

pub(super) struct GameCreaturePlugin;

impl Plugin for GameCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreatureAction>();
        app.init_resource::<CreatureMap>();
        app.add_systems(
            Update,
            (creature_action,).run_if(in_state(info::GlobalStates::Play)),
        );
    }
}

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct Creature(pub Arc<CreatureShared>);

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct CreatureMap(pub HashMap<Id, Creature>);

#[derive(Debug, Clone, Copy)]
pub struct CreatureSystems {
    pub spawn: SystemId<game::Position>,
    pub die: SystemId<Entity>,
    pub damage: SystemId<(Entity, u32)>,
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct CreatureShared {
    #[deref]
    pub systems: CreatureSystems,
    pub cost: u32,
    pub cooldown: f32,
    pub anim: Arc<sprite::FrameArr>,
    pub hitbox: game::HitBox,
}

#[derive(Event, Debug, Clone)]
pub enum CreatureAction {
    Spawn(Id, game::Position),
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
