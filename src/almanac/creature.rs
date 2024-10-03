use crate::prelude::*;

pub(super) struct AlmanacCreaturePlugin;

impl Plugin for AlmanacCreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (switch_to_creature,).run_if(when_state!(almanac_menu)),
        );
        app.add_systems(
            OnEnter(almanac::AlmanacStates::Creature),
            (spawn_creature_info,),
        );
        app.add_systems(
            OnExit(almanac::AlmanacStates::Creature),
            (despawn_creature_info,),
        );
    }
}

#[derive(Resource)]
pub struct SwitchToCreature(pub Id);

#[derive(Component)]
pub struct CreatureInfoMarker;

fn switch_to_creature(
    mut commands: Commands,
    cursor: Res<info::CursorInfo>,
    info: Res<almanac::AlmanacMenuInfo>,
    mut page: ResMut<almanac::AlmanacPage>,
    mut state: ResMut<NextState<almanac::AlmanacStates>>,
) {
    if cursor.left {
        let diff = {
            let mut diff = cursor.pos - info.begin;
            diff.y = -diff.y;
            diff
        };
        let (x, y) = (
            (diff.x / info.each.width) as usize,
            (diff.y / info.each.height) as usize,
        );
        if cursor.pos.x >= info.begin.x && cursor.pos.y <= info.begin.y && x < info.x && y < info.y
        {
            let index = page.0 * info.mul + y * info.x + x;
            let index = info.all.len().saturating_sub(index + 1);
            if let Some(id) = info.all.get(index) {
                commands.insert_resource(SwitchToCreature(*id));
                state.set(almanac::AlmanacStates::Creature);
                page.0 = usize::MAX;
            }
        }
    }
}

fn despawn_creature_info(mut commands: Commands, q_info: Query<Entity, With<CreatureInfoMarker>>) {
    q_info.iter().for_each(|entity| {
        if let Some(commands) = commands.get_entity(entity) {
            commands.despawn_recursive();
        }
    });
}

fn spawn_creature_info(
    mut commands: Commands,
    switch: Res<SwitchToCreature>,
    info: Res<almanac::AlmanacMenuInfo>,
    display: Res<game::Display>,
    map: Res<game::CreatureMap>,
    text: Res<assets::TextCreatures>,
    font: Res<assets::DefaultFont>,
) {
    if let Some((creature, desc)) = map.get(&switch.0).and_then(|creature| {
        text.desc
            .get(id_name(switch.0))
            .map(|desc| (creature, desc))
    }) {
        let begin = info.begin;
        commands.spawn((
            CreatureInfoMarker,
            begin,
            info.each,
            SpriteBundle {
                texture: creature.image.clone(),
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
        let begin = game::Position::new_xy(begin.x, begin.y - info.each.height);
        let name = format!("{}\n", desc.name);
        let details = format!(
            "COST = {}; RECHARGE = {:.1}s\n\n{}",
            creature.cost, creature.cooldown, desc.desc
        );
        commands.spawn((
            CreatureInfoMarker,
            begin,
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new(
                        name,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 0.6 * display.ratio,
                            color: Color::LinearRgba(LinearRgba::new(0.05, 1.0, 1.0, 1.0)),
                        },
                    ),
                    TextSection::new(
                        details,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 0.4 * display.ratio,
                            color: Color::WHITE,
                        },
                    ),
                ]),
                text_2d_bounds: bevy::text::Text2dBounds {
                    size: Vec2::new(
                        LOGICAL.x - display.ratio * info.begin.x * 2.0,
                        f32::INFINITY,
                    ),
                },
                text_anchor: Anchor::TopLeft,
                ..Default::default()
            },
        ));
    };
}
