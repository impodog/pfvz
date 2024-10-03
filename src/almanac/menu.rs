use crate::prelude::*;

pub(super) struct AlmanacMenuPlugin;

impl Plugin for AlmanacMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::MenuStates::Almanac), (init_menu,));
        app.add_systems(OnExit(info::MenuStates::Almanac), (despawn_items,));
        app.add_systems(Update, (change_vis_by_page,).run_if(when_state!(almanac)));
    }
}

#[derive(Resource)]
pub struct AlmanacMenuInfo {
    pub begin: game::Position,
    pub each: game::HitBox,
    pub x: usize,
    pub y: usize,
    pub mul: usize,
    pub pages: usize,
    pub all: Vec<Id>,
}

#[derive(Resource, Default)]
pub struct AlmanacPage(pub usize);

fn init_menu(
    mut commands: Commands,
    mut e_show: EventWriter<almanac::AlmanacShowPicture>,
    save: Res<save::Save>,
) {
    initialize(&level::FIXED_RATIO);
    let begin = {
        let mut begin = LOGICAL / *level::FIXED_RATIO / 3.0;
        begin.x = -begin.x;
        begin
    };
    let y = (LOGICAL.y / *level::FIXED_RATIO) as usize;
    let mul = y * 8;
    let all = save
        .plants
        .iter()
        .chain(save.encounters.iter())
        .copied()
        .collect::<Vec<_>>();
    let pages = all.len().div_ceil(mul);
    let info = AlmanacMenuInfo {
        begin: game::Position::new_xy(begin.x, begin.y),
        each: game::HitBox {
            width: SLOT_SIZE.x,
            height: SLOT_SIZE.y,
        },
        x: 8,
        y,
        mul,
        pages,
        all,
    };

    let mut page = 0;
    let mut column = 0;
    let mut row = 0;
    for id in info.all.iter().rev() {
        let pos = info.begin
            + game::Position::new_xy(
                row as f32 * info.each.width,
                -(column as f32 * info.each.height),
            );
        e_show.send(almanac::AlmanacShowPicture {
            id: *id,
            pos,
            hitbox: info.each,
            vis: if page == 0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
            item: almanac::AlmanacItem { page },
        });

        row += 1;
        if row >= info.x {
            row = 0;
            column += 1;
            if column >= info.y {
                page += 1;
            }
        }
    }

    commands.insert_resource(info);
    commands.insert_resource(AlmanacPage::default());
}

fn despawn_items(mut commands: Commands, q_item: Query<Entity, With<almanac::AlmanacItem>>) {
    q_item.iter().for_each(|entity| {
        if let Some(commands) = commands.get_entity(entity) {
            commands.despawn_recursive();
        }
    });
}

fn change_vis_by_page(
    page: Res<AlmanacPage>,
    mut q_vis: Query<(&mut Visibility, &almanac::AlmanacItem)>,
) {
    if page.is_changed() {
        q_vis.par_iter_mut().for_each(|(mut vis, item)| {
            *vis = if item.page == page.0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        });
    }
}
