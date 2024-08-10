use crate::prelude::*;

pub(super) struct GameSizePlugin;

impl Plugin for GameSizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (add_size, update_size).chain());
        app.add_systems(PreUpdate, (modify_size, modify_relative));
    }
}

/// A helpful layer between hitbox and sprite size
/// Controls the size factor of cropping
#[derive(Component, Default, Debug, Clone)]
pub struct Size {
    pub base: Vec2,
    pub corner: Vec2,
    pub x: game::Factor,
    pub y: game::Factor,
}
impl Size {
    pub fn new(base: Vec2) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    pub fn size(&self) -> Vec2 {
        self.crop(self.base)
    }

    pub fn crop(&self, size: Vec2) -> Vec2 {
        Vec2::new(size.x * self.x.factor(), size.y * self.y.factor())
    }

    pub fn x_factor(&self) -> f32 {
        self.x.factor()
    }

    pub fn y_factor(&self) -> f32 {
        self.y.factor()
    }

    pub fn x_mut(&mut self) -> &mut game::Factor {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut game::Factor {
        &mut self.y
    }
}

fn update_size(
    mut q_size: Query<
        (&Size, &Handle<Image>, &mut Sprite),
        Or<(Changed<Size>, Changed<Handle<Image>>)>,
    >,
    images: Res<Assets<Image>>,
) {
    q_size.par_iter_mut().for_each(|(size, image, mut sprite)| {
        if let Some(image) = images.get(image) {
            sprite.custom_size = Some(size.size());
            let image_size = image.size_f32();
            let new_size = size.crop(image_size);
            let rect = Rect::new(
                size.corner.x,
                size.corner.y,
                size.corner.x + new_size.x,
                size.corner.y + new_size.y,
            );
            sprite.rect = Some(rect);
        } else {
            warn!("No image size available!");
        }
    });
}

fn add_size(
    mut commands: Commands,
    q_hitbox: Query<(Entity, &game::HitBox), Added<game::HitBox>>,
    display: Res<game::Display>,
) {
    q_hitbox.iter().for_each(|(entity, hitbox)| {
        let size = Size::new(Vec2::from(hitbox) * display.ratio);
        commands.entity(entity).try_insert(size);
    });
}

fn modify_size(
    mut q_hitbox: Query<(&game::HitBox, &mut Size), Changed<game::HitBox>>,
    display: Res<game::Display>,
) {
    q_hitbox.par_iter_mut().for_each(|(hitbox, mut size)| {
        size.base = Vec2::from(hitbox) * display.ratio;
    });
}

/// An alternative to `game::Position` when a parent entity affect its positioning
/// This is used with game::Position, and alters the position depending on `Size` of parent
#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct RelativePosition(pub game::Position);
impl RelativePosition {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Self(game::Position::new(x, y, z, r))
    }
}

fn modify_relative(
    mut q_rel: Query<(&Parent, Ref<RelativePosition>, &mut game::Position)>,
    q_size: Query<(&game::HitBox, Ref<Size>)>,
) {
    fn sign(v: f32) -> f32 {
        if v < -f32::EPSILON {
            -1.0
        } else {
            1.0
        }
    }

    q_rel.par_iter_mut().for_each(|(parent, rel, mut pos)| {
        if let Ok((hitbox, size)) = q_size.get(parent.get()) {
            if rel.is_changed() || size.is_changed() {
                *pos = rel.0;
                pos.x -= sign(pos.x) * hitbox.width * (1.0 - size.x_factor()) / 2.0;
                pos.z -= sign(pos.z) * hitbox.height * (1.0 - size.y_factor()) / 2.0;
            }
        }
    });
}
