#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Item
{
    // Foods.
    Apple,
    Cucumber,
    Garlic,
    Orange,
    Potato,
    SweetPotato,
    Tomato,

    // Metals.
    Ore(Metal),
    Ingot(Metal),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metal
{
    Iron,
    Silver,
    Gold,
}
