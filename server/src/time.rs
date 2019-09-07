/// An instant in in-game time.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant
{
    /// The number of seconds since the epoch.
    t: u64,
}

impl Instant
{
    pub const EPOCH: Self = Self{t: 0};
}
