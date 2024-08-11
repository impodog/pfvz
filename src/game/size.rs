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
pub struct SizeCrop {
    pub base: Vec2,
    pub corner: Vec2,
    pub x_crop: game::Factor,
    pub y_crop: game::Factor,
    pub x_stretch: game::Factor,
    pub y_stretch: game::Factor,
}
impl SizeCrop {
    pub fn new(base: Vec2) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    pub fn size(&self) -> Vec2 {
        self.crop_stretch(self.base)
    }

    pub fn crop(&self, size: Vec2) -> Vec2 {
        Vec2::new(size.x * self.x_crop.factor(), size.y * self.y_crop.factor())
    }

    pub fn crop_stretch(&self, size: Vec2) -> Vec2 {
        Vec2::new(
            size.x * self.x_crop.factor() * self.x_stretch.factor(),
            size.y * self.y_crop.factor() * self.y_stretch.factor(),
        )
    }

    pub fn x_multiply(&mut self, factor: f32) {
        self.x_crop.multiply(factor);
        self.x_stretch.multiply(1.0 / factor);
    }

    pub fn y_multiply(&mut self, factor: f32) {
        self.y_crop.multiply(factor);
        self.y_stretch.multiply(1.0 / factor);
    }

    pub fn x_divide(&mut self, factor: f32) {
        self.x_crop.divide(factor);
        self.x_stretch.divide(1.0 / factor);
    }

    pub fn y_divide(&mut self, factor: f32) {
        self.y_crop.divide(factor);
        self.y_stretch.divide(1.0 / factor);
    }
}

fn update_size(
    mut q_size: Query<
        (&SizeCrop, &Handle<Image>, &mut Sprite),
        Or<(Changed<SizeCrop>, Changed<Handle<Image>>)>,
    >,
    images: Res<Assets<Image>>,
) {
    q_size.par_iter_mut().for_each(|(size, image, mut sprite)| {
        sprite.custom_size = Some(size.size());
        if let Some(image) = images.get(image) {
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
            // FIXME: Why is this shown when the game starts into the menu?
            // warn!("No image size available for id {}!", image.id());
        }
    });
}

fn add_size(
    mut commands: Commands,
    q_hitbox: Query<(Entity, &game::HitBox), Added<game::HitBox>>,
    display: Res<game::Display>,
) {
    q_hitbox.iter().for_each(|(entity, hitbox)| {
        let size = SizeCrop::new(Vec2::from(hitbox) * display.ratio);
        commands.entity(entity).try_insert(size);
    });
}

fn modify_size(
    mut q_hitbox: Query<(&game::HitBox, &mut SizeCrop), Changed<game::HitBox>>,
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
    q_size: Query<(&game::HitBox, Ref<SizeCrop>)>,
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
                pos.x -= sign(pos.x) * hitbox.width * (1.0 - size.x_crop.factor()) / 2.0;
                pos.z -= sign(pos.z) * hitbox.height * (1.0 - size.y_crop.factor()) / 2.0;
            }
        }
    });
}
