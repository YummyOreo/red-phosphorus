#![allow(clippy::cast_precision_loss)]
#[allow(clippy::module_name_repetitions)]
pub trait BlockEntity {
    fn set_slot(&mut self, slot_name: SlotName, slot_content: Option<Slot>);
    fn get_slot(&self, slot_name: SlotName) -> Option<Slot>;

    fn get_all(&self) -> Vec<(SlotName, Option<Slot>)>;

    fn get_signal_strength(&self) -> i8;
}

#[derive(Clone)]
pub enum SlotName {
    Num(i32),
    Name(String),
}

#[derive(Clone)]
pub struct Slot {
    pub item: ItemType,
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

#[derive(Clone)]
pub enum ItemType {
    FullStackable,
    FourthStackable,
    SingleStacbable,
}

impl ItemType {
    pub const FULL_STACKABLE_MAX: f32 = 64_f32;
    pub const FOURTH_STACKABLE_MAX: f32 = 18_f32;
    pub const SINGAL_STACKABLE_MAX: f32 = 1_f32;
}

/// Ulits functions for some block entities
/// **You still have to make the block entities, these can just be used to help with the implementation**
pub mod utils {
    use super::{BlockEntity, ItemType, Slot};

    pub const MAX_MID_CONTAINER: i8 = 27;

    pub fn calc_fullness(kind: &ItemType, ammount: f32) -> f32 {
        match *kind {
            ItemType::FullStackable => ammount / ItemType::FULL_STACKABLE_MAX,
            ItemType::FourthStackable => ammount / ItemType::FOURTH_STACKABLE_MAX,
            ItemType::SingleStacbable => ammount / ItemType::SINGAL_STACKABLE_MAX,
        }
    }

    fn get_fullness(slots: Vec<Option<Slot>>) -> f32 {
        // Calculate 'fullness'
        // fullness is the just present items/max items
        let mut fullness = 0_f32;
        for slot in slots {
            let slot = slot.unwrap_or_default();
            fullness += calc_fullness(&slot.item, slot.ammount as f32);
        }
        fullness
    }

    fn calc_strength(fullness: f32, max_slots: f32) -> i8 {
        // Some wierd math from minecraft!
        // casting to i8 should work bc strengths should never go over 15
        #[allow(clippy::cast_possible_truncation)]
        let sum = (fullness / max_slots).mul_add(14_f32, 1_f32).floor();
        let sum = if sum > 15_f32 { 15_i8 } else { sum as i8 };

        return sum;
    }

    /// Calculates the signal strength from a given block entity
    /// Make sure that all slots are "registered"
    /// You can do this by making a slot with `None` as the slot
    ///
    /// There are some exeptions that will have their own functions
    /// See [the wiki](https://minecraft.fandom.com/wiki/Redstone_Comparator#Miscellaneous)
    pub fn calc_signal_strength<T: BlockEntity>(block: &T) -> Option<i8> {
        let slots = block.get_all();
        let max_slots: f32 = slots.len().checked_sub(1)? as f32;

        let slots = slots
            .iter()
            .map(|slot| slot.clone().1)
            .collect::<Vec<Option<Slot>>>();

        let fullness: f32 = get_fullness(slots);
        Some(calc_strength(fullness, max_slots))
    }
}
