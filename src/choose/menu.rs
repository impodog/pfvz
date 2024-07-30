use crate::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct ChooseMenu {
    pub(super) result: Vec<Id>,
    from: BTreeSet<Id>,
    list: Vec<Id>,
    remain: usize,
    remain_max: usize,
}
impl ChooseMenu {
    pub fn from_iter_values(
        result: Vec<Id>,
        from: impl IntoIterator<Item = Id>,
        remain: usize,
    ) -> Self {
        let list = Vec::from_iter(from);
        Self {
            result,
            from: BTreeSet::from_iter(list.iter().cloned()),
            list,
            remain,
            remain_max: remain,
        }
    }

    pub fn contains(&self, id: Id) -> bool {
        self.result.iter().any(|result_id| *result_id == id)
    }

    /// Returns true if the id is added to the result.
    pub fn add(&mut self, id: Id) -> bool {
        if self.remain > 0 && !self.contains(id) && self.from.contains(&id) {
            self.result.push(id);
            self.from.remove(&id);
            self.remain -= 1;
            true
        } else {
            false
        }
    }

    /// Returns true if the id is added to the result.
    pub fn add_index(&mut self, index: usize) -> bool {
        if let Some(id) = self.list.get(index) {
            self.add(*id)
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: Id) -> bool {
        if self.remain < self.remain_max {
            let mut ok = false;
            self.result.retain(|&result_id| {
                if result_id == id {
                    ok = true;
                    false
                } else {
                    true
                }
            });
            if ok {
                self.from.insert(id);
                self.remain += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn remove_index(&mut self, index: usize) -> bool {
        if self.remain < self.remain_max {
            if let Some(id) = self.result.get(index) {
                // Make use of NLL, un-borrow the id
                let id = *id;
                self.result.remove(index);
                self.from.insert(id);
                self.remain += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get(&self, index: usize) -> Option<&Id> {
        self.list.get(index)
    }
}
