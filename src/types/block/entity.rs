#![allow(clippy::cast_precision_loss)]

#[allow(clippy::module_name_repetitions)]
/// Block Entity trait
pub trait BlockEntity {
    /// Get the signal strength when a comparator "reads" from it
    fn get_signal_strength(&self) -> i8;
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Standard way to represent slots. Used in utils. Not stricktly necessary
pub struct Slot {
    /// The item type (ie. full stack (64) or single stack (1))
    pub item: ItemType,
    /// The ammount of the item in the stack
    pub ammount: i32,
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            item: ItemType::FullStackable,
            ammount: 0,
        }
    }
}

impl Slot {
    pub fn new(kind: ItemType, ammount: i32) -> Self {
        Self {
            item: kind,
            ammount,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The type of the item (or slot). Just the max items that can be on the slot
///
/// # Example:
/// ```rust
/// use red_phosphorus::types::block::entity::ItemType;
/// // Upto 64 items
/// let kind: ItemType = ItemType::FullStackable;
/// ```
pub enum ItemType {
    /// Max of 64
    FullStackable,
    /// Max of 16
    FourthStackable,
    /// Max of 1
    SingleStackable,
}
