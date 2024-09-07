use crate::prelude::*;

pub(super) struct AchShowPlugin;

impl Plugin for AchShowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(info::MenuStates::Achievements), (spawn_ach,));
        app.add_systems(
            Update,
            (update_page, exit_ach).run_if(when_state!(achievements)),
        );
        app.add_systems(OnExit(info::MenuStates::Achievements), (despawn_ach,));
    }
}

#[derive(Resource, Debug)]
struct AchPageLayout {
    pages: usize,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub(super) struct AchPageIndex(pub(super) usize);

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct AchPage(usize);

fn spawn_ach(mut commands: Commands, mut e_ach: EventWriter<ach::SpawnAchievement>) {
    let rows = (LOGICAL_WIDTH / ACH_SIZE.x) as usize;
    let cols = (LOGICAL_HEIGHT / ACH_SIZE.y) as usize;
    let num = rows * cols;
    let pages = enum_iterator::all::<ach::AchId>().count().div_ceil(num);
    commands.insert_resource(AchPageLayout { pages });
    commands.init_resource::<AchPage>();
    let mut row = 0;
    let mut col = 0;
    let mut page = 0;
    enum_iterator::all::<ach::AchId>().for_each(|ach| {
        let x = ACH_SIZE.x * row as f32 - LOGICAL_WIDTH / 2.0;
        let y = -ACH_SIZE.y * col as f32 + LOGICAL_HEIGHT / 2.0;
        e_ach.send(ach::SpawnAchievement {
            pos: Vec3::new(x, y, 0.0),
            ach,
            option: ach::SpawnAchievementOption::Page(page),
            anchor: Anchor::TopLeft,
        });

        row += 1;
        if row >= rows {
            row = 0;
            col += 1;
            if col >= cols {
                col = 0;
                page += 1;
            }
        }
    });
}

fn update_page(
    cursor: Res<info::CursorInfo>,
    layout: Res<AchPageLayout>,
    mut page: ResMut<AchPage>,
    mut q_ach: Query<(&mut Visibility, &AchPageIndex), With<ach::AchievementMarker>>,
) {
    if cursor.left {
        page.0 += 1;
        if page.0 >= layout.pages {
            page.0 = 0;
        }
    } else if cursor.right {
        if page.0 == 0 {
            page.0 = layout.pages - 1;
        } else {
            page.0 -= 1;
        }
    }
    if page.is_changed() {
        q_ach.par_iter_mut().for_each(|(mut vis, index)| {
            *vis = if index.0 == page.0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        });
    }
}

fn exit_ach(
    mut state: ResMut<NextState<info::MenuStates>>,
    key: Res<ButtonInput<KeyCode>>,
    cursor: Res<info::CursorInfo>,
) {
    if key.just_pressed(KeyCode::Escape) || cursor.right {
        state.set(info::MenuStates::Main);
    }
}

fn despawn_ach(mut commands: Commands, q_ach: Query<Entity, With<ach::AchievementMarker>>) {
    q_ach.iter().for_each(|ach| {
        commands.entity(ach).despawn_recursive();
    });
}
