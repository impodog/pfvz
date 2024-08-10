use crate::prelude::*;

/// A useful list of factors that can only be multiplied(pushed) and divided(popped) once each
/// It also supports overlapping factors
#[derive(Debug, Clone)]
pub struct Factor {
    factor: f32,
    map: BTreeMap<Orderedf32, usize>,
}

impl Default for Factor {
    fn default() -> Self {
        Self {
            factor: 1.0,
            map: BTreeMap::new(),
        }
    }
}

impl Factor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn factor(&self) -> f32 {
        self.factor
    }

    // Whenever a division by 0.0 occurs, use this to get the accurate result of speed
    pub fn recalculate(&mut self) {
        self.factor = 1.0;
        for (value, times) in self.map.iter() {
            self.factor *= value.0.powi(*times as i32);
        }
    }

    pub fn multiply(&mut self, rate: f32) {
        match self.map.entry(rate.into()) {
            std::collections::btree_map::Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
            std::collections::btree_map::Entry::Occupied(mut occupied) => {
                *occupied.get_mut() += 1;
            }
        }
        self.factor *= rate;
    }

    pub fn divide(&mut self, rate: f32) {
        let remove = match self.map.entry(rate.into()) {
            std::collections::btree_map::Entry::Occupied(mut occupied) => {
                let value = occupied.get().saturating_sub(1);
                *occupied.get_mut() = value;
                value == 0
            }
            _ => false,
        };
        if remove {
            self.map.remove(&rate.into());
            // This prevents dividing by 0
            if rate.abs() <= f32::EPSILON {
                self.recalculate();
            } else {
                self.factor /= rate;
            }
        }
    }
}
