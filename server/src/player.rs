use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlayerId(pub Uuid);
