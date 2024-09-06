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
    Default,
    GettingExciting,
}

impl AchId {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::GettingExciting => "getting_exciting",
        }
    }
}
