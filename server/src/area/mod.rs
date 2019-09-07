/// The area is the smallest unit of space that a player character can find
/// themselves in.
///
/// At the source code level, areas are grouped into larger regions for
/// programmer convenience, but this is not evident in the data structures. You
/// will find a submodule with a capitalized name for each region, which in turn
/// will have a submodule with a capitalized name for each area.

/// Flat enumeration of all areas. The variant names correspond to the region and
/// area names, by the format `{region}_{area}`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AreaId
{
    Silingold_CityGate,
}

impl AreaId
{
    /// The area in which every new player begins.
    pub const INITIAL: Self = AreaId::Silingold_CityGate;
}

/// For each area, there is one value of this type that describes the static
/// aspects of the area; those that do not change during gameplay.
#[derive(Clone, Copy, Debug)]
pub struct AreaStatic
{
    description: &'static str,
}

/// Return the static information about an area. See [AreaStatic] for more
/// information.
///
/// [AreaStatic]: struct.AreaStatic.html
pub fn area_static(area: AreaId) -> AreaStatic
{
    match area {
        AreaId::Silingold_CityGate => Silingold::CityGate::STATIC,
    }
}

pub mod Silingold;
