use std::collections::HashMap;

use area::AreaId;
use interaction::Interaction;
use interaction::InteractionStack;
use inventory::Inventory;
use player::PlayerId;
use time::Instant;

#[derive(Clone, Debug)]
pub struct State
{
    /// The current time in the universe.
    pub time: Instant,

    /// For each player, in which area they are.
    player_areas: HashMap<PlayerId, AreaId>,

    /// For each player, their inventory.
    player_inventories: HashMap<PlayerId, Inventory>,

    /// For each player, their interaction stack.
    player_interaction_stacks: HashMap<PlayerId, InteractionStack>,
}

impl State
{
    pub fn new() -> Self
    {
        Self{
            time: Instant::EPOCH,
            player_areas: HashMap::new(),
            player_inventories: HashMap::new(),
            player_interaction_stacks: HashMap::new(),
        }
    }

    /// Find the area a player is in. If for any reason there is no record of the
    /// player being in any area, return the initial area.
    pub fn player_area(&self, p: PlayerId) -> AreaId
    {
        self.player_areas.get(&p).map(|&a| a)
            .unwrap_or(AreaId::INITIAL)
    }

    /// Instantaneously move a player to a different area.
    pub fn set_player_area(&mut self, p: PlayerId, a: AreaId)
    {
        self.player_areas.insert(p, a);
    }

    /// The current interaction for a player.
    pub fn player_interaction(&self, p: PlayerId) -> Option<&Interaction>
    {
        self.player_interaction_stacks.get(&p).and_then(|s| s.last())
    }
}
