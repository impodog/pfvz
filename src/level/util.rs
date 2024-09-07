use crate::prelude::*;

impl level::LayoutKind {
    /// This returns a index applicable to `PlantLayout`, or usize::MAX if conversion is not
    /// possible
    pub fn position_3d_to_index(&self, pos: &game::Position) -> usize {
        let size = self.size();
        let (x, y) = self.position_3d_to_coordinates(pos);
        if x >= size.0 || y >= size.1 {
            usize::MAX
        } else {
            y * size.0 + x
        }
    }

    pub fn position_2d_to_index(&self, pos: &game::Position) -> usize {
        let size = self.size();
        let (x, y) = self.position_2d_to_coordinates(pos);
        if x >= size.0 || y >= size.1 {
            usize::MAX
        } else {
            y * size.0 + x
        }
    }

    pub fn position_2d_to_coordinates(&self, pos: &game::Position) -> (usize, usize) {
        let size = self.half_size_f32();
        let x = (pos.x + size.0) as usize;
        (x, (pos.y + size.1 - self.get_disp(x)) as usize)
    }

    pub fn position_2d_to_coordinates_checked(
        &self,
        pos: &game::Position,
    ) -> Option<(usize, usize)> {
        let (rows, cols) = self.size();
        let size = self.half_size_f32();
        let x = pos.x + size.0;
        if x >= 0.0 {
            let x = x as usize;
            let y = pos.y + size.1 - self.get_disp(x);
            if y >= 0.0 {
                let y = y as usize;
                if x < rows && y < cols {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn position_3d_to_coordinates(&self, pos: &game::Position) -> (usize, usize) {
        let size = self.half_size_f32();
        let x = (pos.x + size.0) as usize;
        (x, (pos.y + size.1) as usize)
    }

    pub fn position_3d_to_coordinates_checked(
        &self,
        pos: &game::Position,
    ) -> Option<(usize, usize)> {
        let (rows, cols) = self.size();
        let size = self.half_size_f32();
        let x = pos.x + size.0;
        if x >= 0.0 {
            let x = x as usize;
            let y = pos.y + size.1;
            if y >= 0.0 {
                let y = y as usize;
                if x < rows && y < cols {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn coordinates_to_position(&self, x: usize, y: usize) -> game::Position {
        let size = self.half_size_f32();
        game::Position {
            x: x as f32 - size.0 + 0.5,
            y: y as f32 - size.1 + 0.5,
            z: self.get_disp(x),
            r: 0.0,
        }
    }

    pub fn get_disp_of(&self, pos: &game::Position) -> f32 {
        let (x, _y) = self.position_3d_to_coordinates(pos);
        self.get_disp(x)
    }

    pub fn regularize_xy(&self, pos: game::Position) -> game::Position {
        let hsize = self.half_size_f32();
        let pos = pos.move_by(hsize.0, hsize.1);
        game::Position {
            x: pos.x as i32 as f32 - hsize.0 + 0.5,
            y: pos.y as i32 as f32 - hsize.1 + 0.5,
            z: pos.z,
            r: pos.r,
        }
    }

    pub fn regularize_xyzr(&self, pos: &game::Position) -> game::Position {
        let (x, y) = self.position_3d_to_coordinates(pos);
        self.coordinates_to_position(x, y)
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
