use crate::prelude::*;

impl level::LayoutKind {
    /// This returns a index applicable to `PlantLayout`, or usize::MAX if conversion is not
    /// possible
    pub fn position_to_index(&self, pos: &game::Position) -> usize {
        let size = self.half_size_f32();
        let x = (pos.x + size.0) as i32;
        let y = (pos.y + size.1) as i32;
        if let Ok(x) = usize::try_from(x) {
            if let Ok(y) = usize::try_from(y) {
                return y * self.size().0 + x;
            }
        }
        usize::MAX
    }

    /// This returns a index applicable to `PlantLayout`, or usize::MAX if conversion is not
    /// possible
    pub fn position_to_coordinates(&self, pos: &game::Position) -> (usize, usize) {
        let size = self.half_size_f32();
        ((pos.x + size.0) as usize, (pos.y + size.1) as usize)
    }

    pub fn regularize(&self, pos: game::Position) -> game::Position {
        let hsize = self.half_size_f32();
        let pos = pos.move_by(hsize.0, hsize.1);
        game::Position {
            x: pos.x as i32 as f32 - hsize.0 + 0.5,
            y: pos.y as i32 as f32 - hsize.1 + 0.5,
            z: pos.z,
            r: pos.r,
        }
    }

    pub fn same_y(&self, lhs: &game::Position, rhs: &game::Position) -> bool {
        let hsize = self.half_size_f32();
        (lhs.y + hsize.1) as i32 == (rhs.y + hsize.1) as i32
    }
}
