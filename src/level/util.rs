use crate::prelude::*;

impl level::LayoutKind {
    /// This returns a index applicable to `PlantLayout`, or usize::MAX if conversion is not
    /// possible
    pub fn position_to_index(&self, pos: &game::Position) -> usize {
        let size = self.size();
        let (x, y) = self.position_to_coordinates(pos);
        if x >= size.0 || y >= size.1 {
            usize::MAX
        } else {
            y * size.0 + x
        }
    }

    pub fn position_to_coordinates(&self, pos: &game::Position) -> (usize, usize) {
        let size = self.half_size_f32();
        ((pos.x + size.0) as usize, (pos.y + size.1) as usize)
    }

    pub fn coordinates_to_position(&self, x: usize, y: usize) -> game::Position {
        let size = self.half_size_f32();
        game::Position {
            x: x as f32 - size.0 + 0.5,
            y: y as f32 - size.1 + 0.5,
            z: 0.0,
            r: 0.0,
        }
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

    pub fn is_in_bound(&self, pos: &game::Position) -> bool {
        let hsize = self.half_size_f32();
        pos.x >= -hsize.0 && pos.x <= hsize.0 && pos.y >= -hsize.1 && pos.y <= hsize.1
    }

    pub fn same_y(&self, lhs: &game::Position, rhs: &game::Position) -> bool {
        let hsize = self.half_size_f32();
        (lhs.y + hsize.1) as i32 == (rhs.y + hsize.1) as i32
    }
}

impl std::fmt::Display for level::LevelIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.stage, self.level)
    }
}
