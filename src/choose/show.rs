use crate::prelude::*;

pub(super) struct ChooseShowPlugin;

impl Plugin for ChooseShowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(info::PlayStates::Cys),
            (spawn_selection, spawn_buttons),
        );
        app.add_systems(
            Update,
            (modify_page, modify_selection, select_deselect).run_if(when_state!(cys)),
        );
    }
}

#[derive(Component)]
pub struct SelectionMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionInfo {
    pub page: usize,
    pub id: Id,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct SelectionPageInfo {
    pub current: usize,
    pub total: usize,
}
impl SelectionPageInfo {
    pub fn next(&mut self) {
        self.current += 1;
        if self.current >= self.total {
            self.current = 0;
        }
    }

    pub fn prev(&mut self) {
        if self.current == 0 {
            self.current = self.total - 1;
        } else {
            self.current -= 1;
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct SelectionPageSize {
    pub columns: usize,
    pub rows: usize,
    pub each: Vec2,
    // The left top corner of the beginning of selection
    pub begin: game::Position,
}

fn spawn_selection(
    mut commands: Commands,
    display: Res<game::Display>,
    save: Res<save::Save>,
    map: Res<game::CreatureMap>,
    level: Res<level::Level>,
    // Using this preserves the initial selection
    selection: Res<game::Selection>,
) {
    let each = SLOT_SIZE;
    let begin = sprite::SlotIndex(0)
        .into_position(display.ratio)
        .move_by(0.0, -1.0);
    let page_size = SelectionPageSize {
        columns: ((LOGICAL_HEIGHT - begin.y) / each.y) as usize,
        //rows: ((LOGICAL_WIDTH - begin.x) / each.x) as usize,
        rows: 8,
        each,
        begin,
    };
    commands.insert_resource(page_size);
    commands.insert_resource(choose::ChooseMenu::from_iter_values(
        selection.0.clone(),
        save.plants.iter().rev().cloned(),
        level.config.max_select(save.slots.0),
    ));

    let mut page = 0usize;
    let mut col = 0usize;
    let mut row = 0usize;
    for id in save.plants.iter().rev() {
        if row >= page_size.rows {
            row = 0;
            col += 1;
        }
        if col >= page_size.columns {
            col = 0;
            page += 1;
        }
        commands.spawn((
            SelectionMarker,
            SelectionInfo { page, id: *id },
            page_size.begin.move_by(
                page_size.each.x * row as f32,
                -page_size.each.y * col as f32,
            ),
            game::HitBox::from(&SLOT_SIZE),
            SpriteBundle {
                visibility: if page == 0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                texture: map
                    .get(id)
                    .map(|creature| creature.image.clone())
                    .unwrap_or_default(),
                ..Default::default()
            },
        ));
        row += 1;
    }

    commands.insert_resource(SelectionPageInfo {
        current: 0,
        total: page,
    });
}
fn spawn_buttons(
    mut commands: Commands,
    display: Res<game::Display>,
    slots: Res<level::LevelSlots>,
    chunks: Res<assets::SpriteChunks>,
) {
    commands.spawn((
        SelectionMarker,
        game::HitBox::from(&BUTTON_SIZE),
        sprite::SlotIndex(slots.0 + 2).into_position(display.ratio),
        SpriteBundle {
            texture: chunks.next.clone(),
            ..Default::default()
        },
    ));
    commands.spawn((
        SelectionMarker,
        game::HitBox::from(&BUTTON_SIZE),
        sprite::SlotIndex(slots.0 + 4).into_position(display.ratio),
        SpriteBundle {
            texture: chunks.start.clone(),
            ..Default::default()
        },
    ));
}

fn modify_page(mut q_sel: Query<(&mut Visibility, &SelectionInfo)>, page: Res<SelectionPageInfo>) {
    if page.is_changed() {
        q_sel.par_iter_mut().for_each(|(mut visibility, info)| {
            if info.page == page.current {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        });
    }
}

fn modify_selection(
    menu: Res<choose::ChooseMenu>,
    mut event: EventWriter<game::ShowSelectionEvent>,
    mut selection: ResMut<game::Selection>,
) {
    if selection.0 != menu.result {
        selection.0 = menu.result.clone();
        event.send(game::ShowSelectionEvent);
    }
}

#[allow(clippy::too_many_arguments)]
fn select_deselect(
    mut menu: ResMut<choose::ChooseMenu>,
    mut play_state: ResMut<NextState<info::PlayStates>>,
    cursor: Res<info::CursorInfo>,
    page_size: Res<SelectionPageSize>,
    mut page: ResMut<SelectionPageInfo>,
    display: Res<game::Display>,
    slots: Res<level::LevelSlots>,
    level: Res<level::Level>,
) {
    if cursor.left {
        if let Some(index) = sprite::SlotIndex::from_position(cursor.pos, display.ratio) {
            if !menu.remove_index(index.0) {
                // No warnings, because this is mostly player clicking on empty slots
                // Buttons
                if index.0 == slots.0 + 1 || index.0 == slots.0 + 2 {
                    page.next();
                } else if index.0 == slots.0 + 3 || index.0 == slots.0 + 4 {
                    play_state.set(info::PlayStates::Gaming);
                }
            }
        } else {
            let mut pos = cursor.pos.move_by(-page_size.begin.x, -page_size.begin.y);
            pos.y = -pos.y;
            let row = (pos.x / page_size.each.x).round() as usize;
            let col = (pos.y / page_size.each.y).round() as usize;
            let index =
                page.current * page_size.rows * page_size.columns + col * page_size.rows + row;
            if menu
                .get(index)
                .is_some_and(|id| level.config.is_compat(*id))
            {
                menu.add_index(index);
            }
        }
    }
}
