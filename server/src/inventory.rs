use std::collections::HashMap;
use std::num::NonZeroU16;

use item::Item;

#[derive(Clone, Debug)]
pub struct Inventory
{
    items: HashMap<Item, NonZeroU16>,
}
