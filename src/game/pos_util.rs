use crate::prelude::*;

pub(super) struct GamePosUtilPlugin;

impl Plugin for GamePosUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_shadow, move_shadow));
    }
}

/// The logic position of a entity represents the position of their bottom,
/// where the shadow would be
#[derive(Component, Default, Debug, Clone, Copy, Deref, DerefMut)]
pub struct LogicPosition {
    #[deref]
    pub base: game::Position,
    pub disp: game::Position,
}
impl LogicPosition {
    pub fn new(base: game::Position, disp: game::Position) -> Self {
        Self { base, disp }
    }

    pub fn from_base(base: game::Position) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    pub fn with_disp(mut self, disp: game::Position) -> Self {
        self.disp = disp;
        self
    }

    pub fn plus_disp(mut self, disp: game::Position) -> Self {
        self.disp = self.disp + disp;
        self
    }

    pub fn plus(mut self, diff: game::Position) -> Self {
        self.base = self.base + diff;
        self
    }

    pub fn center_of(&self, hitbox: &game::HitBox) -> game::Position {
        self.bottom().move_z(hitbox.height / 2.0)
    }

    pub fn bottom(&self) -> game::Position {
        self.base + self.disp
    }
}

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct ShadowOf(pub Entity);

fn spawn_shadow(
    mut commands: Commands,
    q_parent: Query<
        (Entity, &LogicPosition, &game::HitBox),
        (Added<LogicPosition>, Without<Parent>),
    >,
    chunks: Res<assets::SpriteChunks>,
) {
    q_parent.iter().for_each(|(parent, logic_pos, hitbox)| {
        let height = hitbox.width * 0.3;
        commands.spawn((
            ShadowOf(parent),
            game::HitBox::new(hitbox.width, height),
            logic_pos.base,
            SpriteBundle {
                texture: chunks.shadow.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..Default::default()
            },
        ));
    });
}

fn move_shadow(
    commands: ParallelCommands,
    mut q_shadow: Query<(Entity, &mut game::Position, &ShadowOf, &mut Visibility)>,
    q_parent: Query<Ref<LogicPosition>>,
) {
    q_shadow
        .par_iter_mut()
        .for_each(|(entity, mut pos, shadow, mut vis)| {
            if let Ok(logic_pos) = q_parent.get(shadow.0) {
                if logic_pos.is_changed() || pos.is_added() {
                    *pos = logic_pos.base;
                    pos.z = 0.0;
                    *vis = if logic_pos.base.z < -f32::EPSILON {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    };
                }
            } else {
                commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn_recursive();
                });
            }
        })
}
