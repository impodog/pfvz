use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
)]
#[repr(u32)]
pub enum AchId {
    GettingExciting,
}

impl AchId {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GettingExciting => "getting_exciting",
        }
    }
}
